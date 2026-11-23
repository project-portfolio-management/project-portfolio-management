package io.github.projectportfoliomanagement.mobile.api

import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import okhttp3.MediaType.Companion.toMediaTypeOrNull
import java.util.concurrent.TimeUnit

/** [HttpTransport] over OkHttp (OkHttp because `HttpURLConnection` cannot send `PATCH`, which the move needs). */
public class OkHttpTransport(
    private val client: OkHttpClient = OkHttpClient.Builder()
        .connectTimeout(15, TimeUnit.SECONDS)
        .readTimeout(30, TimeUnit.SECONDS)
        .callTimeout(45, TimeUnit.SECONDS)
        .followRedirects(false)
        .build(),
) : HttpTransport {
    override fun execute(request: HttpRequest): HttpResponse {
        val contentType = request.headers.entries.firstOrNull { it.key.equals("Content-Type", ignoreCase = true) }?.value
        val body = request.body?.toRequestBody(contentType?.toMediaTypeOrNull())
        val builder = Request.Builder().url(request.url).method(request.method, body)
        request.headers.forEach { (name, value) -> builder.header(name, value) }
        client.newCall(builder.build()).execute().use { response ->
            return HttpResponse(response.code, response.body.string())
        }
    }
}
