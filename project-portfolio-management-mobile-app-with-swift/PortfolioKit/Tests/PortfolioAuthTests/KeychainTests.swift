// The Keychain store, against the real Keychain. Opt-in (`PORTFOLIO_KEYCHAIN_TESTS=1`), because on macOS it writes to the
// login keychain of whoever runs `swift test`; it uses its own service name and removes what it wrote.

import Foundation
import PortfolioAuth
import Testing

@Suite(.enabled(if: ProcessInfo.processInfo.environment["PORTFOLIO_KEYCHAIN_TESTS"] == "1"))
struct KeychainTests {
    @Test func writesReadsReplacesAndRemoves() throws {
        let store = KeychainStore(service: "io.github.projectportfoliomanagement.mobile.tests.\(UUID().uuidString)")
        defer { try? store.remove("k") }
        #expect(try store.read("k") == nil)
        try store.write(Data("one".utf8), for: "k")
        #expect(try store.read("k") == Data("one".utf8))
        try store.write(Data("two".utf8), for: "k")
        #expect(try store.read("k") == Data("two".utf8))
        try store.remove("k")
        #expect(try store.read("k") == nil)
        try store.remove("k")
    }
}

@Suite struct UserDefaultsStoreTests {
    @Test func keepsValuesUnderItsPrefix() throws {
        let name = "portfolio-tests-\(UUID().uuidString)"
        let defaults = try #require(UserDefaults(suiteName: name))
        defer { defaults.removePersistentDomain(forName: name) }
        let store = UserDefaultsStore(defaults: defaults, prefix: "p.")
        try store.write(Data("x".utf8), for: "k")
        #expect(defaults.data(forKey: "p.k") == Data("x".utf8))
        #expect(try store.read("k") == Data("x".utf8))
        try store.remove("k")
        #expect(try store.read("k") == nil)
    }
}
