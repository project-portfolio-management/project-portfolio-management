package io.github.projectportfoliomanagement.mobile.app

import io.github.projectportfoliomanagement.mobile.api.HttpRequest
import io.github.projectportfoliomanagement.mobile.api.HttpResponse
import io.github.projectportfoliomanagement.mobile.api.HttpTransport

/** A stub service: answers by "METHOD url-after-the-host", records every request. No network, no running service. */
class RouteTransport : HttpTransport {
    val requests = mutableListOf<HttpRequest>()
    private val routes = mutableMapOf<String, HttpResponse>()

    fun on(method: String, url: String, status: Int, body: String) {
        routes["$method $url"] = HttpResponse(status, body)
    }

    fun sent(method: String, url: String): HttpRequest? = requests.lastOrNull { it.method == method && it.url == url }

    override fun execute(request: HttpRequest): HttpResponse {
        requests += request
        return routes["${request.method} ${request.url}"] ?: HttpResponse(404, """{"status":404,"title":"Not Found"}""")
    }
}
