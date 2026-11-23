// MA-2: discovery and its checks, the authorization request, the redirect, the token requests, refresh and sign-out,
// against a stub provider.

import Foundation
import PortfolioAPI
import PortfolioAuth
import Testing

let settings = OIDCSettings(issuer: "https://id.example.org/realms/ppm", clientID: "ppm-mobile")

let configuration = OIDCConfiguration(
    issuer: "https://id.example.org/realms/ppm",
    authorizationEndpoint: URL(string: "https://id.example.org/realms/ppm/auth")!,
    tokenEndpoint: URL(string: "https://id.example.org/realms/ppm/token")!,
    endSessionEndpoint: URL(string: "https://id.example.org/realms/ppm/logout")!)

/// A provider: the discovery document and the token endpoint, with a switch to refuse refreshes.
final class Provider: HTTPTransport, @unchecked Sendable {
    let lock = NSLock()
    var sent: [HTTPRequest] = []
    var document = #"""
        {"issuer":"https://id.example.org/realms/ppm",
         "authorization_endpoint":"https://id.example.org/realms/ppm/auth",
         "token_endpoint":"https://id.example.org/realms/ppm/token",
         "end_session_endpoint":"https://id.example.org/realms/ppm/logout",
         "code_challenge_methods_supported":["plain","S256"],"jwks_uri":"https://id.example.org/certs"}
        """#
    var refuseRefresh = false
    var issued = 0

    func send(_ request: HTTPRequest) async throws -> HTTPResponse {
        lock.withLock { sent.append(request) }
        if request.url.path().hasSuffix("/.well-known/openid-configuration") {
            return HTTPResponse(status: 200, body: Data(lock.withLock { document }.utf8))
        }
        if request.url.path().hasSuffix("/token") {
            let form = String(decoding: request.body ?? Data(), as: UTF8.self)
            if form.contains("grant_type=refresh_token") && lock.withLock({ refuseRefresh }) {
                return HTTPResponse(status: 400, body: Data(#"{"error":"invalid_grant"}"#.utf8))
            }
            let n = lock.withLock { issued += 1; return issued }
            return HTTPResponse(
                status: 200,
                body: Data(
                    #"{"access_token":"access-\#(n)","token_type":"Bearer","expires_in":300,"refresh_token":"refresh-\#(n)","id_token":"id-\#(n)"}"#
                        .utf8))
        }
        return HTTPResponse(status: 404)
    }

    func form(_ index: Int) -> [String: String] {
        let body = String(decoding: lock.withLock { sent[index] }.body ?? Data(), as: UTF8.self)
        return Dictionary(
            body.split(separator: "&").map {
                let pair = $0.split(separator: "=", maxSplits: 1).map { String($0).removingPercentEncoding ?? String($0) }
                return (pair[0], pair.count > 1 ? pair[1] : "")
            }, uniquingKeysWith: { $1 })
    }
}

/// A clock the tests move.
final class Clock: @unchecked Sendable {
    let lock = NSLock()
    var now = Date(timeIntervalSince1970: 1_800_000_000)
    func advance(_ seconds: TimeInterval) { lock.withLock { now += seconds } }
    func read() -> Date { lock.withLock { now } }
}

func query(_ url: URL) -> [String: String] {
    Dictionary(
        (URLComponents(url: url, resolvingAgainstBaseURL: false)?.queryItems ?? []).map { ($0.name, $0.value ?? "") },
        uniquingKeysWith: { $1 })
}

@Suite struct DiscoveryTests {
    @Test func theDocumentURLKeepsTheIssuersPath() {
        #expect(
            OIDCDiscovery.documentURL(issuer: URL(string: "https://id.example.org/realms/ppm/")!).absoluteString
                == "https://id.example.org/realms/ppm/.well-known/openid-configuration")
        #expect(
            OIDCDiscovery.documentURL(issuer: URL(string: "https://id.example.org")!).absoluteString
                == "https://id.example.org/.well-known/openid-configuration")
    }

    @Test func discovers() async throws {
        let found = try await OIDCDiscovery.discover(settings: settings, transport: Provider())
        #expect(found == configuration.withMethods(["plain", "S256"]))
    }

    @Test func theIssuerMustEqualTheOneSet() async {
        let provider = Provider()
        provider.document = provider.document.replacingOccurrences(
            of: #""issuer":"https://id.example.org/realms/ppm""#, with: #""issuer":"https://evil.example.org""#)
        await #expect(throws: AuthError.issuerMismatch(found: "https://evil.example.org")) {
            try await OIDCDiscovery.discover(settings: settings, transport: provider)
        }
    }

    @Test func aTrailingSlashIsAnotherIssuer() async {
        var slashed = settings
        slashed.issuer += "/"
        await #expect(throws: AuthError.issuerMismatch(found: settings.issuer)) {
            try await OIDCDiscovery.discover(settings: slashed, transport: Provider())
        }
    }

    @Test func theIssuerMustBeHTTPS() async {
        var plain = settings
        plain.issuer = "http://id.example.org/realms/ppm"
        await #expect(throws: AuthError.issuerNotAllowed) {
            try await OIDCDiscovery.discover(settings: plain, transport: Provider())
        }
    }

    @Test func aLoopbackIssuerMayBeHTTP() throws {
        let local = OIDCSettings(issuer: "http://127.0.0.1:8080/realms/dev", clientID: "c")
        let doc = OIDCConfiguration(
            issuer: local.issuer, authorizationEndpoint: URL(string: "http://127.0.0.1:8080/auth")!,
            tokenEndpoint: URL(string: "http://127.0.0.1:8080/token")!)
        try OIDCDiscovery.check(doc, settings: local)
    }

    @Test func everyEndpointMustBeHTTPS() {
        let doc = OIDCConfiguration(
            issuer: settings.issuer, authorizationEndpoint: configuration.authorizationEndpoint,
            tokenEndpoint: URL(string: "http://id.example.org/token")!)
        #expect(throws: AuthError.endpointNotAllowed) { try OIDCDiscovery.check(doc, settings: settings) }
    }

    @Test func aProviderWithoutS256IsRefused() {
        #expect(throws: AuthError.s256Unsupported) {
            try OIDCDiscovery.check(configuration.withMethods(["plain"]), settings: settings)
        }
        // A provider that does not list its methods is tried.
        #expect(throws: Never.self) { try OIDCDiscovery.check(configuration.withMethods(nil), settings: settings) }
    }

    @Test func aMissingDocumentFails() async {
        let provider = Provider()
        provider.document = "not json"
        await #expect(throws: AuthError.discoveryFailed) {
            try await OIDCDiscovery.discover(settings: settings, transport: provider)
        }
    }
}

@Suite struct AuthorizationTests {
    let pending = AuthorizationFlow.begin(
        configuration: configuration, settings: settings, state: "st4te", verifier: PKCETests.verifier)

    @Test func theRequestCarriesPKCEAndState() {
        #expect(pending.url.absoluteString.hasPrefix("https://id.example.org/realms/ppm/auth?"))
        #expect(
            query(pending.url) == [
                "response_type": "code", "client_id": "ppm-mobile",
                "redirect_uri": "io.github.projectportfoliomanagement.mobile:/oauth2redirect", "scope": "openid offline_access",
                "state": "st4te", "code_challenge": PKCETests.challenge, "code_challenge_method": "S256",
            ])
    }

    @Test func theCodeComesBackWithTheState() throws {
        let redirect = URL(string: "io.github.projectportfoliomanagement.mobile:/oauth2redirect?code=abc&state=st4te")!
        #expect(try AuthorizationFlow.code(from: redirect, pending: pending) == "abc")
    }

    @Test func anotherStateIsRefused() {
        let redirect = URL(string: "io.github.projectportfoliomanagement.mobile:/oauth2redirect?code=abc&state=other")!
        #expect(throws: AuthError.stateMismatch) { try AuthorizationFlow.code(from: redirect, pending: pending) }
        let none = URL(string: "io.github.projectportfoliomanagement.mobile:/oauth2redirect?code=abc")!
        #expect(throws: AuthError.stateMismatch) { try AuthorizationFlow.code(from: none, pending: pending) }
    }

    @Test func theProvidersErrorIsReported() {
        let redirect = URL(
            string: "io.github.projectportfoliomanagement.mobile:/oauth2redirect?error=access_denied&error_description=no&state=st4te")!
        #expect(throws: AuthError.provider(error: "access_denied", description: "no")) {
            try AuthorizationFlow.code(from: redirect, pending: pending)
        }
    }

    @Test func aRedirectWithoutACodeIsRefused() {
        let redirect = URL(string: "io.github.projectportfoliomanagement.mobile:/oauth2redirect?state=st4te")!
        #expect(throws: AuthError.missingCode) { try AuthorizationFlow.code(from: redirect, pending: pending) }
    }

    @Test func theExchangeSendsTheVerifier() {
        let request = TokenRequests.exchange(code: "a+b", pending: pending, configuration: configuration, clientID: "ppm-mobile")
        #expect(request.method == .post)
        #expect(request.url == configuration.tokenEndpoint)
        #expect(request.headers["Content-Type"] == "application/x-www-form-urlencoded")
        #expect(
            String(decoding: request.body!, as: UTF8.self)
                == "grant_type=authorization_code&code=a%2Bb&redirect_uri=io.github.projectportfoliomanagement.mobile%3A%2Foauth2redirect&client_id=ppm-mobile&code_verifier=\(PKCETests.verifier)"
        )
    }

    @Test func theEndSessionURL() throws {
        let url = try #require(TokenRequests.endSession(configuration: configuration, settings: settings, idToken: "id-1"))
        #expect(
            query(url) == [
                "client_id": "ppm-mobile", "id_token_hint": "id-1",
                "post_logout_redirect_uri": "io.github.projectportfoliomanagement.mobile:/oauth2redirect",
            ])
        #expect(TokenRequests.endSession(configuration: configuration.withoutEndSession(), settings: settings, idToken: nil) == nil)
    }
}

@Suite struct SessionTests {
    func signedIn(_ provider: Provider, _ clock: Clock, store: InMemoryStore = InMemoryStore()) async throws -> AuthSession {
        let session = AuthSession(settings: settings, transport: provider, store: store, now: { clock.read() })
        let (pending, scheme) = try await session.beginSignIn()
        #expect(scheme == "io.github.projectportfoliomanagement.mobile")
        let redirect = URL(string: "\(settings.redirectURI)?code=the-code&state=\(pending.state)")!
        try await session.completeSignIn(redirect: redirect, pending: pending)
        return session
    }

    @Test func signsInAndKeepsTheTokensInTheStore() async throws {
        let provider = Provider()
        let store = InMemoryStore()
        let session = try await signedIn(provider, Clock(), store: store)
        #expect(try await session.accessToken() == "access-1")
        #expect(await session.hasTokens)
        let exchange = provider.form(1)
        #expect(exchange["grant_type"] == "authorization_code")
        #expect(exchange["code"] == "the-code")
        #expect(PKCE.isValidVerifier(exchange["code_verifier"] ?? ""))
        #expect(try store.read(AuthSession.tokensKey) != nil)
    }

    @Test func aWrongStateStoresNothing() async throws {
        let store = InMemoryStore()
        let session = AuthSession(settings: settings, transport: Provider(), store: store)
        let (pending, _) = try await session.beginSignIn()
        let redirect = URL(string: "\(settings.redirectURI)?code=c&state=forged")!
        await #expect(throws: AuthError.stateMismatch) { try await session.completeSignIn(redirect: redirect, pending: pending) }
        #expect(try store.read(AuthSession.tokensKey) == nil)
    }

    @Test func aTokenNearExpiryIsRefreshed() async throws {
        let provider = Provider()
        let clock = Clock()
        let session = try await signedIn(provider, clock)
        clock.advance(200)
        #expect(try await session.accessToken() == "access-1")
        clock.advance(50)  // 50 s left: inside the 60 s margin
        #expect(try await session.accessToken() == "access-2")
        let refresh = provider.form(provider.sent.count - 1)
        #expect(refresh["grant_type"] == "refresh_token")
        #expect(refresh["refresh_token"] == "refresh-1")
        #expect(refresh["client_id"] == "ppm-mobile")
    }

    @Test func aFailedRefreshSignsOut() async throws {
        let provider = Provider()
        let clock = Clock()
        let store = InMemoryStore()
        let session = try await signedIn(provider, clock, store: store)
        provider.refuseRefresh = true
        clock.advance(290)
        await #expect(throws: AuthError.signedOut) { try await session.accessToken() }
        #expect(try store.read(AuthSession.tokensKey) == nil)
        await #expect(throws: AuthError.signedOut) { try await session.accessToken() }
    }

    @Test func signOutForgetsTheTokensAndNamesTheEndSession() async throws {
        let store = InMemoryStore()
        let session = try await signedIn(Provider(), Clock(), store: store)
        let url = try #require(await session.signOut())
        #expect(url.absoluteString.hasPrefix("https://id.example.org/realms/ppm/logout?"))
        #expect(query(url)["id_token_hint"] == "id-1")
        #expect(try store.read(AuthSession.tokensKey) == nil)
        await #expect(throws: AuthError.signedOut) { try await session.accessToken() }
    }

    @Test func aRefreshWithoutANewRefreshTokenKeepsTheOld() {
        let previous = TokenSet(accessToken: "a", refreshToken: "r", idToken: "i", expiresAt: nil)
        let response = try! JSONDecoder().decode(TokenResponse.self, from: Data(#"{"access_token":"b","expires_in":60}"#.utf8))
        let next = TokenSet(response, now: Date(timeIntervalSince1970: 0), previous: previous)
        #expect(next == TokenSet(accessToken: "b", refreshToken: "r", idToken: "i", expiresAt: Date(timeIntervalSince1970: 60)))
    }
}

extension OIDCConfiguration {
    func withMethods(_ methods: [String]?) -> OIDCConfiguration {
        OIDCConfiguration(
            issuer: issuer, authorizationEndpoint: authorizationEndpoint, tokenEndpoint: tokenEndpoint,
            endSessionEndpoint: endSessionEndpoint, codeChallengeMethodsSupported: methods)
    }

    func withoutEndSession() -> OIDCConfiguration {
        OIDCConfiguration(
            issuer: issuer, authorizationEndpoint: authorizationEndpoint, tokenEndpoint: tokenEndpoint,
            endSessionEndpoint: nil, codeChallengeMethodsSupported: codeChallengeMethodsSupported)
    }
}
