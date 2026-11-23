// MA-6: each call's request (method, path, headers, body) and the decoding of each answer, against a stub transport.
package io.github.projectportfoliomanagement.mobile.api

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class ApiClientTest {
    private val base = "https://ppm.example.org/"
    private val json = mapOf("Accept" to "application/json", "Accepts-version" to "1.0")
    private val authed = json + ("Authorization" to "Bearer t0ken")

    private fun client(vararg answers: Any, token: String? = "t0ken") =
        StubTransport(*answers).let { it to ApiClient(base, it) { token } }

    private fun <T> ok(result: ApiResult<T>): T {
        assertTrue("expected Ok, got $result", result is ApiResult.Ok)
        return (result as ApiResult.Ok).value
    }

    @Test fun readinessIsAPlainGetWithoutAToken() {
        val (stub, api) = client(HttpResponse(200, """{"ok":true}"""))
        ok(api.checkReady())
        assertEquals(HttpRequest("GET", "https://ppm.example.org/_health/ready", json, null), stub.requests.single())
    }

    @Test fun listPlansSendsTheHeadersAndReadsRefs() {
        val (stub, api) = client(HttpResponse(200, """[{"pid":"p1","name":"Alpha","extra":1},{"pid":"p2","name":"Beta"}]"""))
        assertEquals(listOf(PlanRef("p1", "Alpha"), PlanRef("p2", "Beta")), ok(api.listPlans()))
        assertEquals(HttpRequest("GET", "https://ppm.example.org/api/plans", authed, null), stub.requests.single())
    }

    @Test fun noTokenSendsNoAuthorization() {
        val (stub, api) = client(HttpResponse(200, "[]"), token = null)
        ok(api.listPlans())
        assertEquals(json, stub.requests.single().headers)
    }

    @Test fun searchEncodesTheQuery() {
        val (stub, api) = client(HttpResponse(200, """[{"pid":"p1","name":"Café & co"}]"""))
        assertEquals(listOf(PlanRef("p1", "Café & co")), ok(api.searchPlans("café & co")))
        assertEquals("https://ppm.example.org/api/plans/search?q=caf%C3%A9%20%26%20co", stub.requests.single().url)
        assertEquals("GET", stub.requests.single().method)
    }

    @Test fun getPlanReadsACustomStatusAndIgnoresUnknownFields() {
        val body = """{"name":"Alpha","kind":"Project","code":"A-1","status":{"Custom":"Paused for audit"},
            "start_date":"2027-01-04","target_date":"2027-06-30","owner_org_name":"Ops","goals":[{"title":"g"}],
            "a_field_from_a_newer_service":{"x":[1,2]}}"""
        val (stub, api) = client(HttpResponse(200, body))
        val plan = ok(api.getPlan("p 1"))
        assertEquals(
            Plan(
                name = "Alpha", kind = "Project", code = "A-1", status = PlanStatus.Custom("Paused for audit"),
                ownerOrgName = "Ops", startDate = "2027-01-04", targetDate = "2027-06-30",
            ),
            plan,
        )
        assertEquals("https://ppm.example.org/api/plans/p%201", stub.requests.single().url)
    }

    @Test fun getPlanReadsAUnitStatusAndNulls() {
        val (_, api) = client(HttpResponse(200, """{"name":"Beta","status":"OnHold","kind":null,"code":null}"""))
        val plan = ok(api.getPlan("p2"))
        assertEquals(PlanStatus.OnHold, plan.status)
        assertNull(plan.kind)
    }

    @Test fun childPlansUseTheParentQuery() {
        val (stub, api) = client(HttpResponse(200, """[{"pid":"c1","name":"Child"}]"""))
        assertEquals(listOf(PlanRef("c1", "Child")), ok(api.childPlans("p1")))
        assertEquals(HttpRequest("GET", "https://ppm.example.org/api/plans?parent=p1", authed, null), stub.requests.single())
    }

    @Test fun tasksReadTheRowAndIgnoreUnknownFields() {
        val body = """[{"pid":"t1","title":"Write it","description":null,"status":"blocked","assignee_ref":"person:7",
            "points":3,"due_date":"2027-02-01","deadline":null,"sprint_pid":null,"created_at":"2027-01-01T00:00:00Z",
            "status_changed_at":"2027-01-02T00:00:00Z","done_at":null,"blocked_days":4,"is_hammock":false,"new":"x"}]"""
        val (stub, api) = client(HttpResponse(200, body))
        assertEquals(
            listOf(Task("t1", "Write it", null, "blocked", "person:7", 3, "2027-02-01", 4)),
            ok(api.tasks("p1")),
        )
        assertEquals(HttpRequest("GET", "https://ppm.example.org/api/plans/p1/tasks", authed, null), stub.requests.single())
    }

    @Test fun moveIsAPatchWithTheStatus() {
        val (stub, api) = client(HttpResponse(200, """{"pid":"t1","title":"Write it","status":"done"}"""))
        assertEquals("done", ok(api.moveTask("p1", "t1", "done")).status)
        assertEquals(
            HttpRequest(
                "PATCH", "https://ppm.example.org/api/plans/p1/tasks/t1",
                authed + ("Content-Type" to "application/json"), """{"status":"done"}""",
            ),
            stub.requests.single(),
        )
    }

    @Test fun createPostsOnlyTheFieldsSet() {
        val (stub, api) = client(HttpResponse(201, """{"pid":"p9","name":"Gamma"}"""))
        val created = ok(api.createPlan(NewPlan(name = "Gamma", status = PlanStatus.Active, startDate = "2027-03-01")))
        assertEquals(PlanRef("p9", "Gamma"), created)
        assertEquals(
            HttpRequest(
                "POST", "https://ppm.example.org/api/plans",
                authed + ("Content-Type" to "application/json"),
                """{"name":"Gamma","status":"Active","start_date":"2027-03-01"}""",
            ),
            stub.requests.single(),
        )
    }

    @Test fun createWritesACustomStatusAsAnObject() {
        val (stub, api) = client(HttpResponse(201, """{"pid":"p9","name":"Gamma"}"""))
        ok(api.createPlan(NewPlan(name = "Gamma", kind = "Program", status = PlanStatus.Custom("Shaping"), code = "G")))
        assertEquals("""{"name":"Gamma","kind":"Program","status":{"Custom":"Shaping"},"code":"G"}""", stub.requests.single().body)
    }

    @Test fun anUnreadableSuccessIsUnexpectedNotACrash() {
        val (_, api) = client(HttpResponse(200, "<html>proxy</html>"))
        val result = api.listPlans()
        assertTrue(result is ApiResult.Failed && result.error is ApiError.Unexpected)
    }
}
