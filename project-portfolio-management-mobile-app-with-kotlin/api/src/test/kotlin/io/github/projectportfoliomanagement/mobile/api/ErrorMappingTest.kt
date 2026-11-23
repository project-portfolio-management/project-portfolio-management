// MA-6: one test per row of MA-3, through the client against a stub transport.
package io.github.projectportfoliomanagement.mobile.api

import org.junit.Assert.assertEquals
import org.junit.Test
import java.io.IOException
import java.net.SocketTimeoutException

class ErrorMappingTest {
    private fun failed(result: ApiResult<*>): ApiError = (result as ApiResult.Failed).error

    private fun api(vararg answers: Any) = ApiClient("https://ppm.example.org", StubTransport(*answers)) { "t" }

    private fun problem(status: Int, detail: String, code: String = "x") =
        HttpResponse(status, """{"type":"about:blank","title":"T","status":$status,"detail":"$detail","code":"$code"}""")

    @Test fun row401SignsOut() {
        assertEquals(ApiError.Unauthorized, failed(api(problem(401, "missing token")).listPlans()))
    }

    @Test fun row403NamesTheRule() {
        assertEquals(
            ApiError.Forbidden("rule writers-only: access=write required"),
            failed(api(problem(403, "rule writers-only: access=write required")).moveTask("p", "t", "done")),
        )
        assertEquals(ApiError.Forbidden(null), failed(api(HttpResponse(403, "")).listPlans()))
    }

    @Test fun row404IsNotThere() {
        assertEquals(ApiError.NotFound, failed(api(problem(404, "not found")).getPlan("gone")))
    }

    @Test fun row409OnCreateIsADuplicate() {
        assertEquals(ApiError.Duplicate, failed(api(problem(409, "duplicate")).createPlan(NewPlan("A"))))
        // A 409 anywhere else is not "a plan like this exists".
        assertEquals(ApiError.Unexpected(409, "duplicate"), failed(api(problem(409, "duplicate")).moveTask("p", "t", "done")))
    }

    @Test fun row422ListsEveryProblemWithItsField() {
        val detail = "name is required; parent_ref: \\\"x\\\" is not a valid UUID; code: exceeds 4096 characters; WIP limit reached"
        assertEquals(
            ApiError.Invalid(
                listOf(
                    FieldProblem("name", "name is required"),
                    FieldProblem("parent_ref", "parent_ref: \"x\" is not a valid UUID"),
                    FieldProblem("code", "code: exceeds 4096 characters"),
                    FieldProblem(null, "WIP limit reached"),
                ),
            ),
            failed(api(problem(422, detail, "validation")).createPlan(NewPlan(""))),
        )
    }

    @Test fun row422PrefersTheErrorsMember() {
        val body = """{"status":422,"detail":"two problems","errors":{"name":["is required"],"start_date":"not a date"}}"""
        assertEquals(
            ApiError.Invalid(listOf(FieldProblem("name", "is required"), FieldProblem("start_date", "not a date"))),
            failed(api(HttpResponse(422, body)).createPlan(NewPlan(""))),
        )
    }

    @Test fun rowNoNetworkTimeoutAnd5xxAreUnreachable() {
        assertEquals(ApiError.Unreachable(ApiError.Unreachable.Reason.Network), failed(api(IOException("no route")).listPlans()))
        assertEquals(ApiError.Unreachable(ApiError.Unreachable.Reason.Timeout), failed(api(SocketTimeoutException()).listPlans()))
        assertEquals(ApiError.Unreachable(ApiError.Unreachable.Reason.Server), failed(api(problem(503, "down")).listPlans()))
        assertEquals(ApiError.Unreachable(ApiError.Unreachable.Reason.Server), failed(api(HttpResponse(500, "")).listPlans()))
    }

    @Test fun otherStatusesAreUnexpected() {
        assertEquals(ApiError.Unexpected(400, "bad body"), failed(api(problem(400, "bad body")).createPlan(NewPlan("A"))))
    }
}
