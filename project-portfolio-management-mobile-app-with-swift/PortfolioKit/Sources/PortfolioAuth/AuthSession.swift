// One person's sign-in with one provider (spec/mobile-apps.md MA-2): sign in through the system browser, hand out an access
// token (refreshing one near expiry), and sign out. When a refresh fails the tokens are forgotten and the person is signed
// out.

import Foundation
import PortfolioAPI

public actor AuthSession {
    public static let tokensKey = "tokens"

    public let settings: OIDCSettings
    private let transport: any HTTPTransport
    private let tokens: StoredValue<TokenSet>
    private let now: @Sendable () -> Date
    private var configuration: OIDCConfiguration?
    private var refreshing: Task<TokenSet, any Error>?

    public init(
        settings: OIDCSettings, transport: any HTTPTransport, store: any KeyValueStore,
        now: @escaping @Sendable () -> Date = { Date() }
    ) {
        self.settings = settings
        self.transport = transport
        self.tokens = StoredValue(store: store, key: Self.tokensKey)
        self.now = now
    }

    /// Whether tokens are stored (they may still fail to refresh).
    public var hasTokens: Bool { tokens.load() != nil }

    /// The provider's configuration, fetched once per session.
    public func discover() async throws(AuthError) -> OIDCConfiguration {
        if let configuration { return configuration }
        let found = try await OIDCDiscovery.discover(settings: settings, transport: transport)
        configuration = found
        return found
    }

    /// The first half of a sign-in: discovery, and the authorization request to open in the system browser
    /// (`ASWebAuthenticationSession` on iOS) together with the scheme its redirect comes back on.
    public func beginSignIn() async throws(AuthError) -> (pending: PendingAuthorization, callbackScheme: String) {
        let configuration = try await discover()
        guard let scheme = settings.callbackScheme else { throw .invalidRedirectURI }
        return (AuthorizationFlow.begin(configuration: configuration, settings: settings), scheme)
    }

    /// The second half: the `state` check, the code exchange (with the verifier), and storing the tokens.
    public func completeSignIn(redirect: URL, pending: PendingAuthorization) async throws(AuthError) {
        let configuration = try await discover()
        let code = try AuthorizationFlow.code(from: redirect, pending: pending)
        let request = TokenRequests.exchange(
            code: code, pending: pending, configuration: configuration, clientID: settings.clientID)
        let set = TokenSet(try await tokenResponse(request), now: now())
        do { try tokens.save(set) } catch { throw .tokenRequestFailed }
    }

    /// A current access token, refreshed first when it is near expiry. Throws `.signedOut` when there is none or the refresh
    /// fails, having forgotten the tokens.
    public func accessToken() async throws(AuthError) -> String {
        guard let set = tokens.load() else { throw .signedOut }
        guard set.isNearExpiry(now: now()) else { return set.accessToken }
        do {
            return try await refreshed(set).accessToken
        } catch {
            tokens.clear()
            throw .signedOut
        }
    }

    /// Forgets the tokens, and returns the provider's end-session URL to open when it has one.
    public func signOut() async -> URL? {
        let idToken = tokens.load()?.idToken
        tokens.clear()
        guard let configuration = try? await discover() else { return nil }
        return TokenRequests.endSession(configuration: configuration, settings: settings, idToken: idToken)
    }

    /// One refresh at a time: a second caller waits for the first.
    private func refreshed(_ set: TokenSet) async throws -> TokenSet {
        if let refreshing { return try await refreshing.value }
        guard let refreshToken = set.refreshToken else { throw AuthError.signedOut }
        let task = Task { () async throws -> TokenSet in
            let configuration = try await self.discover()
            let request = TokenRequests.refresh(
                refreshToken: refreshToken, configuration: configuration, clientID: self.settings.clientID)
            let next = TokenSet(try await self.tokenResponse(request), now: self.now(), previous: set)
            try self.tokens.save(next)
            return next
        }
        refreshing = task
        defer { refreshing = nil }
        return try await task.value
    }

    private func tokenResponse(_ request: HTTPRequest) async throws(AuthError) -> TokenResponse {
        guard let response = try? await transport.send(request), (200..<300).contains(response.status),
            let decoded = try? JSONDecoder().decode(TokenResponse.self, from: response.body)
        else { throw .tokenRequestFailed }
        return decoded
    }
}
