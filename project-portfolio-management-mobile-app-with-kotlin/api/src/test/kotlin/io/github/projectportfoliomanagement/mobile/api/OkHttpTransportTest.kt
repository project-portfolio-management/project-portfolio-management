// The real transport against a local MockWebServer: the method (PATCH included), headers and body reach the wire, and any
// status comes back as an answer rather than an exception.
package io.github.projectportfoliomanagement.mobile.api

import mockwebserver3.MockResponse
import mockwebserver3.MockWebServer
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test

class OkHttpTransportTest {
    private val server = MockWebServer()

    @Before fun start() = server.start()

    @After fun stop() = server.close()

    @Test fun aPatchCarriesItsHeadersAndBody() {
        server.enqueue(MockResponse.Builder().code(200).body("""{"pid":"t1","title":"T","status":"done"}""").build())
        val api = ApiClient(server.url("/").toString(), OkHttpTransport()) { "tok" }
        val result = api.moveTask("p1", "t1", "done")
        assertEquals("done", (result as ApiResult.Ok).value.status)
        val recorded = server.takeRequest()
        assertEquals("PATCH", recorded.method)
        assertEquals("/api/plans/p1/tasks/t1", recorded.url.encodedPath)
        assertEquals("application/json", recorded.headers["Accept"])
        assertEquals("1.0", recorded.headers["Accepts-version"])
        assertEquals("Bearer tok", recorded.headers["Authorization"])
        assertEquals("""{"status":"done"}""", recorded.body?.utf8())
    }

    @Test fun anErrorStatusIsAnAnswer() {
        server.enqueue(MockResponse.Builder().code(404).body("""{"status":404}""").build())
        val api = ApiClient(server.url("/").toString(), OkHttpTransport())
        assertEquals(ApiResult.Failed(ApiError.NotFound), api.getPlan("x"))
    }
}
