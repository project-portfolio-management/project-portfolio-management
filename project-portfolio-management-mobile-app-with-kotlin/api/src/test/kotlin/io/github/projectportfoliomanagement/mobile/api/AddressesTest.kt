package io.github.projectportfoliomanagement.mobile.api

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class AddressesTest {
    @Test fun developmentAddressesAreLoopbackAndTheEmulatorHost() {
        listOf("http://localhost:5150", "http://127.0.0.1", "http://127.1.2.3:80", "http://[::1]:5150", "http://10.0.2.2:5150")
            .forEach { assertTrue(it, Addresses.isDevelopment(it)) }
        listOf("http://10.0.2.3", "https://ppm.example.org", "http://localhost.example.org", "ftp://localhost", "nonsense")
            .forEach { assertFalse(it, Addresses.isDevelopment(it)) }
    }

    @Test fun httpsIsRequiredExceptForDevelopmentAddresses() {
        assertTrue(Addresses.isAllowed("https://ppm.example.org"))
        assertTrue(Addresses.isAllowed("http://10.0.2.2:5150"))
        assertFalse(Addresses.isAllowed("http://ppm.example.org"))
        assertFalse(Addresses.isAllowed("ppm.example.org"))
        assertFalse(Addresses.isAllowed(""))
    }

    @Test fun normaliseDropsTheTrailingSlash() {
        assertEquals("https://a.example/x", Addresses.normalise(" https://a.example/x/ "))
    }
}
