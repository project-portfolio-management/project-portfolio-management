// The first slice's calls (../spec/mobile-apps.md MA-1). Every request sends `Accept: application/json` and
// `Accepts-version: 1.0`, and the bearer token when [tokens] has one.
package io.github.projectportfoliomanagement.mobile.api

import kotlinx.serialization.KSerializer
import kotlinx.serialization.builtins.ListSerializer
import java.io.IOException

/** Gives the access token for the next request, refreshing it first when it is near expiry; null when signed out. */
public fun interface AccessTokenSource {
    public fun accessToken(): String?
}

public class ApiClient(
    baseUrl: String,
    private val transport: HttpTransport,
    private val tokens: AccessTokenSource = AccessTokenSource { null },
) {
    private val base = Addresses.normalise(baseUrl)

    /** `GET /_health/ready`: is this the service, and is it ready (MA-1.1). */
    public fun checkReady(): ApiResult<Unit> = call("GET", "/_health/ready", null, authorised = false) { }

    /** `GET /api/plans`: the active plans. */
    public fun listPlans(): ApiResult<List<PlanRef>> =
        call("GET", "/api/plans", null) { WireJson.decodeFromString(planRefs, it) }

    /** `GET /api/plans/search?q=`. */
    public fun searchPlans(query: String): ApiResult<List<PlanRef>> =
        call("GET", "/api/plans/search?q=${percentEncode(query)}", null) { WireJson.decodeFromString(planRefs, it) }

    /** `GET /api/plans/{pid}`. */
    public fun getPlan(pid: String): ApiResult<Plan> =
        call("GET", "/api/plans/${percentEncode(pid)}", null) { WireJson.decodeFromString(Plan.serializer(), it) }

    /** `GET /api/plans?parent={pid}`: the plan's direct children. */
    public fun childPlans(pid: String): ApiResult<List<PlanRef>> =
        call("GET", "/api/plans?parent=${percentEncode(pid)}", null) { WireJson.decodeFromString(planRefs, it) }

    /** `GET /api/plans/{pid}/tasks`. */
    public fun tasks(pid: String): ApiResult<List<Task>> =
        call("GET", "/api/plans/${percentEncode(pid)}/tasks", null) { WireJson.decodeFromString(taskList, it) }

    /** `PATCH /api/plans/{pid}/tasks/{tid}` with `{"status": …}`; answers the moved task. */
    public fun moveTask(pid: String, taskPid: String, status: String): ApiResult<Task> = call(
        "PATCH",
        "/api/plans/${percentEncode(pid)}/tasks/${percentEncode(taskPid)}",
        WireJson.encodeToString(TaskMove.serializer(), TaskMove(status)),
    ) { WireJson.decodeFromString(Task.serializer(), it) }

    /** `POST /api/plans`; answers `{pid, name}`. A `409` is [ApiError.Duplicate]. */
    public fun createPlan(plan: NewPlan): ApiResult<PlanRef> = call(
        "POST",
        "/api/plans",
        WireJson.encodeToString(NewPlan.serializer(), plan),
        isCreate = true,
    ) { WireJson.decodeFromString(PlanRef.serializer(), it) }

    /** The request [call] sends; public so tests can state it exactly. */
    public fun request(method: String, path: String, body: String?, authorised: Boolean = true): HttpRequest {
        val headers = linkedMapOf("Accept" to "application/json", "Accepts-version" to "1.0")
        if (body != null) headers["Content-Type"] = "application/json"
        if (authorised) tokens.accessToken()?.let { headers["Authorization"] = "Bearer $it" }
        return HttpRequest(method, base + path, headers, body)
    }

    private fun <T> call(
        method: String,
        path: String,
        body: String?,
        authorised: Boolean = true,
        isCreate: Boolean = false,
        decode: (String) -> T,
    ): ApiResult<T> {
        val response = try {
            transport.execute(request(method, path, body, authorised))
        } catch (e: IOException) {
            return ApiResult.Failed(ErrorMapping.fromException(e))
        }
        if (response.status !in 200..299) {
            return ApiResult.Failed(ErrorMapping.fromResponse(response.status, response.body, isCreate))
        }
        return try {
            ApiResult.Ok(decode(response.body))
        } catch (e: IllegalArgumentException) {
            // kotlinx.serialization's SerializationException is an IllegalArgumentException.
            ApiResult.Failed(ApiError.Unexpected(0, e.message))
        }
    }

    private companion object {
        val planRefs: KSerializer<List<PlanRef>> = ListSerializer(PlanRef.serializer())
        val taskList: KSerializer<List<Task>> = ListSerializer(Task.serializer())
    }
}
