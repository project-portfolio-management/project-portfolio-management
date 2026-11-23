// Where the app keeps what it keeps (spec/mobile-apps.md MA-2): the tokens in the iOS Keychain, readable after the first
// unlock and never leaving this device (`kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly`); the server settings (no
// secret) in user defaults. No token, plan or task data is written anywhere else; a sign-in under way is held in memory.

import Foundation
import Security

/// A small store of values by key.
public protocol KeyValueStore: Sendable {
    func read(_ key: String) throws -> Data?
    func write(_ data: Data, for key: String) throws
    func remove(_ key: String) throws
}

public struct KeychainError: Error, Sendable, Hashable {
    public let status: OSStatus
}

/// Generic-password Keychain items under one service name.
public struct KeychainStore: KeyValueStore {
    public let service: String

    public init(service: String = "io.github.projectportfoliomanagement.mobile") {
        self.service = service
    }

    private func query(_ key: String) -> [String: Any] {
        [kSecClass as String: kSecClassGenericPassword, kSecAttrService as String: service, kSecAttrAccount as String: key]
    }

    public func read(_ key: String) throws -> Data? {
        var q = query(key)
        q[kSecReturnData as String] = true
        q[kSecMatchLimit as String] = kSecMatchLimitOne
        var item: CFTypeRef?
        let status = SecItemCopyMatching(q as CFDictionary, &item)
        if status == errSecItemNotFound { return nil }
        guard status == errSecSuccess else { throw KeychainError(status: status) }
        return item as? Data
    }

    public func write(_ data: Data, for key: String) throws {
        let attributes: [String: Any] = [
            kSecValueData as String: data,
            kSecAttrAccessible as String: kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly,
        ]
        let status = SecItemUpdate(query(key) as CFDictionary, attributes as CFDictionary)
        if status == errSecItemNotFound {
            let added = SecItemAdd(query(key).merging(attributes) { $1 } as CFDictionary, nil)
            guard added == errSecSuccess else { throw KeychainError(status: added) }
        } else if status != errSecSuccess {
            throw KeychainError(status: status)
        }
    }

    public func remove(_ key: String) throws {
        let status = SecItemDelete(query(key) as CFDictionary)
        guard status == errSecSuccess || status == errSecItemNotFound else { throw KeychainError(status: status) }
    }
}

/// Values in user defaults under one key prefix: for the server settings only, never a token.
public struct UserDefaultsStore: KeyValueStore, @unchecked Sendable {
    private let defaults: UserDefaults
    private let prefix: String

    public init(defaults: UserDefaults = .standard, prefix: String = "io.github.projectportfoliomanagement.mobile.") {
        self.defaults = defaults
        self.prefix = prefix
    }

    public func read(_ key: String) throws -> Data? { defaults.data(forKey: prefix + key) }
    public func write(_ data: Data, for key: String) throws { defaults.set(data, forKey: prefix + key) }
    public func remove(_ key: String) throws { defaults.removeObject(forKey: prefix + key) }
}

/// A store in memory, for tests and the stub mode; forgotten when the app quits.
public final class InMemoryStore: KeyValueStore, @unchecked Sendable {
    private let lock = NSLock()
    private var values: [String: Data] = [:]

    public init() {}

    public func read(_ key: String) throws -> Data? { lock.withLock { values[key] } }
    public func write(_ data: Data, for key: String) throws { lock.withLock { values[key] = data } }
    public func remove(_ key: String) throws { _ = lock.withLock { values.removeValue(forKey: key) } }
}

/// A Codable value in a `KeyValueStore` under one key.
public struct StoredValue<Value: Codable & Sendable>: Sendable {
    public let store: any KeyValueStore
    public let key: String

    public init(store: any KeyValueStore, key: String) {
        self.store = store
        self.key = key
    }

    /// The value, or nil when there is none or it cannot be read.
    public func load() -> Value? {
        guard let data = try? store.read(key) else { return nil }
        return try? JSONDecoder().decode(Value.self, from: data)
    }

    public func save(_ value: Value) throws {
        try store.write(JSONEncoder().encode(value), for: key)
    }

    public func clear() {
        try? store.remove(key)
    }
}
