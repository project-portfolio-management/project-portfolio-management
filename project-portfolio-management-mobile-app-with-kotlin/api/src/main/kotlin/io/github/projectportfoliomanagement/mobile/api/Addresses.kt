// MA-2: HTTPS is required for the service and the issuer, except for a development address (a loopback address or the
// Android emulator's host); only a development address offers "Continue without signing in".
package io.github.projectportfoliomanagement.mobile.api

import java.net.URI

public object Addresses {
    /** The emulator's alias for the host machine's loopback interface. */
    public const val EMULATOR_HOST: String = "10.0.2.2"

    /** The host of [url], or null when it is not an absolute http(s) URL with a host. */
    public fun host(url: String): String? = runCatching {
        val uri = URI(url.trim())
        val scheme = uri.scheme?.lowercase()
        if (scheme != "http" && scheme != "https") null else uri.host?.removePrefix("[")?.removeSuffix("]")?.lowercase()
    }.getOrNull()

    /** True for `localhost`, `127.0.0.0/8`, `::1` and `10.0.2.2`. */
    public fun isDevelopment(url: String): Boolean {
        val host = host(url) ?: return false
        return host == "localhost" || host == "::1" || host == "0:0:0:0:0:0:0:1" || host == EMULATOR_HOST ||
            Regex("""127(\.\d{1,3}){3}""").matches(host)
    }

    /** True when [url] is an absolute https URL, or http to a development address. */
    public fun isAllowed(url: String): Boolean {
        val host = host(url) ?: return false
        if (host.isEmpty()) return false
        val scheme = URI(url.trim()).scheme.lowercase()
        return scheme == "https" || isDevelopment(url)
    }

    /** [url] trimmed, without a trailing slash. */
    public fun normalise(url: String): String = url.trim().trimEnd('/')
}
