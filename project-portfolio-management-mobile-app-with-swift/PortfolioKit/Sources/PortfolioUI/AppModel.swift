// The app's state across screens: the server settings, whether the person is signed in, and the client every screen calls
// through. A `401` from any screen comes here and signs the person out (spec/mobile-apps.md MA-3).

import Foundation
import Observation
import PortfolioAPI
import PortfolioAuth
import PortfolioStub

/// The server screen's settings (MA-1.1), kept in user defaults (they hold no secret; MA-2).
public struct ServerSettings: Sendable, Hashable, Codable {
    public var serviceURL: URL
    /// The sign-in settings; nil only for a development address used without signing in.
    public var oidc: OIDCSettings?
    /// The person chose **Continue without signing in** (a development address only).
    public var withoutSignIn: Bool

    public init(serviceURL: URL, oidc: OIDCSettings?, withoutSignIn: Bool = false) {
        self.serviceURL = serviceURL
        self.oidc = oidc
        self.withoutSignIn = withoutSignIn
    }
}

/// What the app runs against: the real network, the Keychain for the tokens and user defaults for the settings; or the
/// stub and memory (the UI tests' `-stub`).
public struct AppEnvironment: Sendable {
    /// Where the UI tests' stub starts.
    public enum StubStart: Sendable {
        case plans, server, signIn
    }

    public let transport: any HTTPTransport
    /// The tokens' store: the Keychain.
    public let store: any KeyValueStore
    /// The server settings' store: user defaults.
    public let preferences: any KeyValueStore
    public let isStub: Bool

    public init(
        transport: any HTTPTransport, store: any KeyValueStore, preferences: (any KeyValueStore)? = nil,
        isStub: Bool = false
    ) {
        self.transport = transport
        self.store = store
        self.preferences = preferences ?? store
        self.isStub = isStub
    }

    public static func live() -> AppEnvironment {
        AppEnvironment(transport: URLSessionTransport(), store: KeychainStore(), preferences: UserDefaultsStore())
    }

    /// The stub service at a loopback address, in memory, starting at `start`.
    public static func stub(start: StubStart) -> AppEnvironment {
        let store = InMemoryStore()
        let settings: ServerSettings? =
            switch start {
            case .plans: ServerSettings(serviceURL: stubURL, oidc: nil, withoutSignIn: true)
            case .signIn: ServerSettings(serviceURL: stubURL, oidc: nil, withoutSignIn: false)
            case .server: nil
            }
        if let settings { try? StoredValue<ServerSettings>(store: store, key: AppModel.settingsKey).save(settings) }
        return AppEnvironment(transport: StubTransport(), store: store, isStub: true)
    }

    public static let stubURL = URL(string: "http://127.0.0.1:5150")!
}

@MainActor @Observable
public final class AppModel {
    public nonisolated static let settingsKey = "server-settings"

    public enum Phase: Sendable, Hashable {
        case server, signIn, ready
    }

    public private(set) var phase: Phase
    public private(set) var settings: ServerSettings?
    /// Why the sign-in screen is showing, when it is not the first sign-in.
    public var signInNotice: String?
    /// Whether a sign-in is running.
    public private(set) var signingIn = false
    /// The last sign-in failure.
    public var signInError: String?

    public let environment: AppEnvironment
    @ObservationIgnored private var auth: AuthSession?
    @ObservationIgnored private let storedSettings: StoredValue<ServerSettings>

    public init(environment: AppEnvironment) {
        self.environment = environment
        storedSettings = StoredValue(store: environment.preferences, key: Self.settingsKey)
        let settings = storedSettings.load()
        self.settings = settings
        phase = .server
        if let settings {
            auth = settings.oidc.map {
                AuthSession(settings: $0, transport: environment.transport, store: environment.store)
            }
            phase = .signIn
            if settings.withoutSignIn && ServerAddress.isDevelopment(settings.serviceURL) {
                phase = .ready
            } else if (try? environment.store.read(AuthSession.tokensKey)) != nil {
                phase = .ready
            }
        }
    }

    /// The client for the current settings: no token without sign-in, else the session's (refreshed when near expiry).
    public var client: PortfolioClient? {
        guard let settings else { return nil }
        if settings.withoutSignIn {
            return PortfolioClient(baseURL: settings.serviceURL, transport: environment.transport)
        }
        let auth = auth
        return PortfolioClient(baseURL: settings.serviceURL, transport: environment.transport) {
            guard let auth else { throw AuthError.signedOut }
            return try await auth.accessToken()
        }
    }

    /// Whether **Continue without signing in** is offered: only for a development (loopback) address (MA-2).
    public var offersWithoutSignIn: Bool {
        settings.map { ServerAddress.isDevelopment($0.serviceURL) } ?? false
    }

    /// Stores new settings, forgetting any tokens for the old ones, and goes to the sign-in screen.
    public func save(_ new: ServerSettings) {
        try? environment.store.remove(AuthSession.tokensKey)
        var new = new
        new.withoutSignIn = false
        try? storedSettings.save(new)
        settings = new
        auth = new.oidc.map { AuthSession(settings: $0, transport: environment.transport, store: environment.store) }
        signInNotice = nil
        signInError = nil
        phase = .signIn
    }

    public func changeServer() {
        phase = .server
    }

    /// Signs in through the system browser; `authenticate` opens the URL and returns the redirect.
    public func signIn(authenticate: @MainActor (URL, String) async throws -> URL) async {
        guard let auth else {
            signInError = Copy.signInNoIssuer.text
            return
        }
        signingIn = true
        signInError = nil
        defer { signingIn = false }
        do {
            let (pending, scheme) = try await auth.beginSignIn()
            let redirect: URL
            do { redirect = try await authenticate(pending.url, scheme) } catch { throw AuthError.cancelled }
            try await auth.completeSignIn(redirect: redirect, pending: pending)
            signInNotice = nil
            phase = .ready
        } catch let error as AuthError {
            signInError = ErrorPresentation.of(error)
        } catch {
            signInError = ErrorPresentation.of(AuthError.cancelled)
        }
    }

    /// Uses a development service with no token (MA-2); refused for any other address.
    public func continueWithoutSigningIn() {
        guard var settings, offersWithoutSignIn else { return }
        settings.withoutSignIn = true
        try? storedSettings.save(settings)
        self.settings = settings
        signInNotice = nil
        signInError = nil
        phase = .ready
    }

    /// Forgets the tokens, then opens the provider's end-session page when it has one (`openEndSession`).
    public func signOut(openEndSession: @MainActor (URL) async -> Void = { _ in }) async {
        let endSession = await auth?.signOut()
        leave(notice: Copy.signInSignedOut.text)
        if let endSession { await openEndSession(endSession) }
    }

    /// Handles the errors every screen shares. Returns true when the error was a `401`, which signs the person out and
    /// shows the sign-in screen; the screen then shows nothing of its own.
    @discardableResult
    public func handle(_ error: APIError) -> Bool {
        guard error == .unauthorized else { return false }
        let wasWithoutSignIn = settings?.withoutSignIn ?? false
        try? environment.store.remove(AuthSession.tokensKey)
        leave(notice: wasWithoutSignIn ? Copy.signInRequired.text : Copy.signInSessionEnded.text)
        return true
    }

    private func leave(notice: String) {
        if var settings, settings.withoutSignIn {
            settings.withoutSignIn = false
            try? storedSettings.save(settings)
            self.settings = settings
        }
        signInNotice = notice
        signInError = nil
        phase = .signIn
    }
}
