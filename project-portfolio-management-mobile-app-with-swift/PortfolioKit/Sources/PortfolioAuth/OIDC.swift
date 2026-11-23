// OIDC discovery, the authorization request, the redirect, and the token endpoint's requests (spec/mobile-apps.md MA-2).
// The app is a public client: no client secret, the authorization-code flow with PKCE S256, in the system browser.

import Foundation
import PortfolioAPI

/// The sign-in settings a person enters on the server screen.
public struct OIDCSettings: Sendable, Hashable, Codable {
    public static let defaultRedirectURI = "io.github.projectportfoliomanagement.mobile:/oauth2redirect"
    public static let defaultScopes = ["openid", "offline_access"]

    public var issuer: String
    public var clientID: String
    public var redirectURI: String
    public var scopes: [String]

    public init(issuer: String, clientID: String, redirectURI: String = defaultRedirectURI, scopes: [String] = defaultScopes) {
        self.issuer = issuer
        self.clientID = clientID
        self.redirectURI = redirectURI
        self.scopes = scopes
    }

    /// The redirect URI's scheme, which the browser session listens for.
    public var callbackScheme: String? { URL(string: redirectURI)?.scheme }
}

/// The members of `/.well-known/openid-configuration` the app reads.
public struct OIDCConfiguration: Sendable, Hashable, Codable {
    public let issuer: String
    public let authorizationEndpoint: URL
    public let tokenEndpoint: URL
    public let endSessionEndpoint: URL?
    public let codeChallengeMethodsSupported: [String]?

    enum CodingKeys: String, CodingKey {
        case issuer
        case authorizationEndpoint = "authorization_endpoint"
        case tokenEndpoint = "token_endpoint"
        case endSessionEndpoint = "end_session_endpoint"
        case codeChallengeMethodsSupported = "code_challenge_methods_supported"
    }

    public init(
        issuer: String, authorizationEndpoint: URL, tokenEndpoint: URL, endSessionEndpoint: URL? = nil,
        codeChallengeMethodsSupported: [String]? = ["S256"]
    ) {
        self.issuer = issuer
        self.authorizationEndpoint = authorizationEndpoint
        self.tokenEndpoint = tokenEndpoint
        self.endSessionEndpoint = endSessionEndpoint
        self.codeChallengeMethodsSupported = codeChallengeMethodsSupported
    }
}

public enum AuthError: Error, Sendable, Hashable {
    /// The issuer is not an HTTPS URL (or a development address).
    case issuerNotAllowed
    /// The discovery document could not be fetched, or is not one.
    case discoveryFailed
    /// The document's `issuer` is not the issuer set; carries the document's.
    case issuerMismatch(found: String)
    /// An endpoint in the document is not HTTPS (or a development address).
    case endpointNotAllowed
    /// The provider says it does not support S256.
    case s256Unsupported
    /// The redirect's `state` is not the one sent.
    case stateMismatch
    /// The provider answered the redirect with an error (`access_denied`, …).
    case provider(error: String, description: String?)
    /// The redirect carried no code.
    case missingCode
    /// The person closed the browser sheet.
    case cancelled
    /// The token endpoint refused the code or the refresh token, or could not be reached.
    case tokenRequestFailed
    /// There are no tokens, or they could not be refreshed: the person must sign in.
    case signedOut
    /// The redirect URI is not a URL with a scheme.
    case invalidRedirectURI
}

public enum OIDCDiscovery {
    /// `{issuer}/.well-known/openid-configuration`, keeping any path the issuer has (a Keycloak realm, for example).
    public static func documentURL(issuer: URL) -> URL {
        var components = URLComponents(url: issuer, resolvingAgainstBaseURL: false) ?? URLComponents()
        var path = components.percentEncodedPath
        while path.hasSuffix("/") { path.removeLast() }
        components.percentEncodedPath = path + "/.well-known/openid-configuration"
        components.query = nil
        components.fragment = nil
        return components.url ?? issuer
    }

    public static func request(issuer: URL) -> HTTPRequest {
        HTTPRequest(method: .get, url: documentURL(issuer: issuer), headers: ["Accept": "application/json"])
    }

    /// Fetches and checks the provider's configuration: HTTPS, `issuer` equal to the one set, S256 not refused.
    public static func discover(settings: OIDCSettings, transport: any HTTPTransport) async throws(AuthError)
        -> OIDCConfiguration
    {
        guard let issuer = URL(string: settings.issuer), ServerAddress.isAllowed(issuer) else {
            throw .issuerNotAllowed
        }
        let response: HTTPResponse
        do { response = try await transport.send(request(issuer: issuer)) } catch { throw .discoveryFailed }
        guard (200..<300).contains(response.status) else { throw .discoveryFailed }
        let configuration: OIDCConfiguration
        do {
            configuration = try JSONDecoder().decode(OIDCConfiguration.self, from: response.body)
        } catch { throw .discoveryFailed }
        try check(configuration, settings: settings)
        return configuration
    }

    /// The checks `discover` makes on a document.
    public static func check(_ configuration: OIDCConfiguration, settings: OIDCSettings) throws(AuthError) {
        guard configuration.issuer == settings.issuer else { throw .issuerMismatch(found: configuration.issuer) }
        let endpoints = [configuration.authorizationEndpoint, configuration.tokenEndpoint]
            + (configuration.endSessionEndpoint.map { [$0] } ?? [])
        guard endpoints.allSatisfy(ServerAddress.isAllowed) else { throw .endpointNotAllowed }
        if let methods = configuration.codeChallengeMethodsSupported, !methods.contains("S256") {
            throw .s256Unsupported
        }
    }
}

/// One sign-in in progress: what was sent, so the redirect can be checked and the code exchanged.
public struct PendingAuthorization: Sendable, Hashable {
    public let url: URL
    public let state: String
    public let verifier: String
    public let redirectURI: String
}

public enum AuthorizationFlow {
    /// The authorization request (RFC 6749 §4.1.1 with RFC 7636 §4.3).
    public static func begin(
        configuration: OIDCConfiguration, settings: OIDCSettings, state: String = OAuthState.random(),
        verifier: String = PKCE.randomVerifier()
    ) -> PendingAuthorization {
        var components = URLComponents(url: configuration.authorizationEndpoint, resolvingAgainstBaseURL: false)
            ?? URLComponents()
        let items = [
            URLQueryItem(name: "response_type", value: "code"),
            URLQueryItem(name: "client_id", value: settings.clientID),
            URLQueryItem(name: "redirect_uri", value: settings.redirectURI),
            URLQueryItem(name: "scope", value: settings.scopes.joined(separator: " ")),
            URLQueryItem(name: "state", value: state),
            URLQueryItem(name: "code_challenge", value: PKCE.challenge(for: verifier)),
            URLQueryItem(name: "code_challenge_method", value: "S256"),
        ]
        components.queryItems = (components.queryItems ?? []) + items
        // URLComponents leaves `+` alone in a query; a provider may read it as a space.
        components.percentEncodedQuery = components.percentEncodedQuery?.replacingOccurrences(of: "+", with: "%2B")
        return PendingAuthorization(
            url: components.url ?? configuration.authorizationEndpoint, state: state, verifier: verifier,
            redirectURI: settings.redirectURI)
    }

    /// The code from the redirect, once its `state` is checked.
    public static func code(from redirect: URL, pending: PendingAuthorization) throws(AuthError) -> String {
        let items = URLComponents(url: redirect, resolvingAgainstBaseURL: false)?.queryItems ?? []
        func value(_ name: String) -> String? { items.first { $0.name == name }?.value }
        guard OAuthState.matches(expected: pending.state, received: value("state")) else { throw .stateMismatch }
        if let error = value("error") { throw .provider(error: error, description: value("error_description")) }
        guard let code = value("code"), !code.isEmpty else { throw .missingCode }
        return code
    }
}

/// The token endpoint's answer (RFC 6749 §5.1).
public struct TokenResponse: Sendable, Hashable, Decodable {
    public let accessToken: String
    public let tokenType: String?
    public let expiresIn: Int?
    public let refreshToken: String?
    public let idToken: String?

    enum CodingKeys: String, CodingKey {
        case accessToken = "access_token"
        case tokenType = "token_type"
        case expiresIn = "expires_in"
        case refreshToken = "refresh_token"
        case idToken = "id_token"
    }
}

/// The tokens the app keeps, in the Keychain only.
public struct TokenSet: Sendable, Hashable, Codable {
    public var accessToken: String
    public var refreshToken: String?
    public var idToken: String?
    /// When the access token expires; nil when the provider did not say.
    public var expiresAt: Date?

    public init(accessToken: String, refreshToken: String? = nil, idToken: String? = nil, expiresAt: Date? = nil) {
        self.accessToken = accessToken
        self.refreshToken = refreshToken
        self.idToken = idToken
        self.expiresAt = expiresAt
    }

    /// The set a token response gives at `now`; a refresh that returns no new refresh token keeps the old one.
    public init(_ response: TokenResponse, now: Date, previous: TokenSet? = nil) {
        accessToken = response.accessToken
        refreshToken = response.refreshToken ?? previous?.refreshToken
        idToken = response.idToken ?? previous?.idToken
        expiresAt = response.expiresIn.map { now.addingTimeInterval(TimeInterval($0)) }
    }

    /// Whether the access token expires within `margin` of `now`.
    public func isNearExpiry(now: Date, margin: TimeInterval = 60) -> Bool {
        guard let expiresAt else { return false }
        return expiresAt.timeIntervalSince(now) < margin
    }
}

public enum TokenRequests {
    /// Exchanges the code (RFC 6749 §4.1.3), sending the verifier (RFC 7636 §4.5).
    public static func exchange(code: String, pending: PendingAuthorization, configuration: OIDCConfiguration, clientID: String)
        -> HTTPRequest
    {
        form(
            configuration.tokenEndpoint,
            [
                ("grant_type", "authorization_code"), ("code", code), ("redirect_uri", pending.redirectURI),
                ("client_id", clientID), ("code_verifier", pending.verifier),
            ])
    }

    /// Refreshes the access token (RFC 6749 §6).
    public static func refresh(refreshToken: String, configuration: OIDCConfiguration, clientID: String) -> HTTPRequest {
        form(
            configuration.tokenEndpoint,
            [("grant_type", "refresh_token"), ("refresh_token", refreshToken), ("client_id", clientID)])
    }

    /// The end-session URL (OpenID RP-Initiated Logout), or nil when the provider has none.
    public static func endSession(configuration: OIDCConfiguration, settings: OIDCSettings, idToken: String?) -> URL? {
        guard let endpoint = configuration.endSessionEndpoint,
            var components = URLComponents(url: endpoint, resolvingAgainstBaseURL: false)
        else { return nil }
        var items = components.queryItems ?? []
        items.append(URLQueryItem(name: "client_id", value: settings.clientID))
        items.append(URLQueryItem(name: "post_logout_redirect_uri", value: settings.redirectURI))
        if let idToken { items.append(URLQueryItem(name: "id_token_hint", value: idToken)) }
        components.queryItems = items
        return components.url
    }

    static func form(_ url: URL, _ fields: [(String, String)]) -> HTTPRequest {
        let allowed = CharacterSet(charactersIn: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~")
        let body = fields.map { name, value in
            "\(name)=\(value.addingPercentEncoding(withAllowedCharacters: allowed) ?? value)"
        }.joined(separator: "&")
        return HTTPRequest(
            method: .post, url: url,
            headers: ["Accept": "application/json", "Content-Type": "application/x-www-form-urlencoded"],
            body: Data(body.utf8))
    }
}
