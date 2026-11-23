package io.github.projectportfoliomanagement.mobile.api

import java.io.IOException

/** Records every request and answers from a queue: an [HttpResponse], or an [IOException] to throw. */
class StubTransport(vararg answers: Any) : HttpTransport {
    val requests = mutableListOf<HttpRequest>()
    private val queue = ArrayDeque(answers.toList())

    override fun execute(request: HttpRequest): HttpResponse {
        requests += request
        return when (val next = queue.removeFirstOrNull() ?: error("no answer queued for $request")) {
            is HttpResponse -> next
            is IOException -> throw next
            else -> error("not an answer: $next")
        }
    }
}
