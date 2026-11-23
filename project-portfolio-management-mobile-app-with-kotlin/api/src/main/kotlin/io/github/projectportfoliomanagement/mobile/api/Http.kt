// The one seam between the app and the network: a request in, a status and a body out. Tests stub it; the app uses
// [OkHttpTransport].
package io.github.projectportfoliomanagement.mobile.api

import java.io.IOException

/** One HTTP request. [headers] carries `Content-Type` when [body] is set. */
public data class HttpRequest(
    val method: String,
    val url: String,
    val headers: Map<String, String>,
    val body: String? = null,
)

/** One HTTP answer, of any status. */
public data class HttpResponse(val status: Int, val body: String)

/** Sends a request and returns the answer, whatever its status; throws [IOException] when there is no answer. */
public fun interface HttpTransport {
    @Throws(IOException::class)
    public fun execute(request: HttpRequest): HttpResponse
}

/** Percent-encodes [value] for a path segment or a query value (RFC 3986 unreserved characters are kept). */
public fun percentEncode(value: String): String {
    val out = StringBuilder()
    for (byte in value.toByteArray(Charsets.UTF_8)) {
        val c = byte.toInt() and 0xff
        val ch = c.toChar()
        if (ch in 'A'..'Z' || ch in 'a'..'z' || ch in '0'..'9' || ch == '-' || ch == '.' || ch == '_' || ch == '~') {
            out.append(ch)
        } else {
            out.append('%').append("0123456789ABCDEF"[c shr 4]).append("0123456789ABCDEF"[c and 0xf])
        }
    }
    return out.toString()
}
