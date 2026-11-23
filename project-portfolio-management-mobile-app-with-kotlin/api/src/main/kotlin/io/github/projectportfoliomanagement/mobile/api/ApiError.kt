// MA-3: every answer that is not a success, mapped to what the app must tell the person. The words are the app's
// (strings.xml); this file only decides which case it is.
package io.github.projectportfoliomanagement.mobile.api

import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.contentOrNull
import java.io.IOException
import java.io.InterruptedIOException

/** One problem from a `422`: the field it names (a wire name such as `name` or `start_date`), or null, and its words. */
public data class FieldProblem(val field: String?, val message: String)

public sealed interface ApiError {
    /** `401`: sign out and ask the person to sign in again. */
    public data object Unauthorized : ApiError

    /** `403`: not allowed; [rule] is the deciding rule the service names in the body, when it names one. */
    public data class Forbidden(val rule: String?) : ApiError

    /** `404`: the plan or task is not there (deleted or merged). */
    public data object NotFound : ApiError

    /** `409` on create: a plan like this already exists. */
    public data object Duplicate : ApiError

    /** `422`: every problem the service lists, each with the field it names where it names one. */
    public data class Invalid(val problems: List<FieldProblem>) : ApiError

    /** No network, a timeout, or a `5xx`: the service could not be reached; offer to try again. */
    public data class Unreachable(val reason: Reason) : ApiError {
        public enum class Reason { Network, Timeout, Server }
    }

    /**
     * Anything MA-3 has no row for: a `400`, a `409` outside create, another `4xx`, or a success whose body the app
     * cannot read. [status] is the HTTP status (0 when the body could not be read).
     */
    public data class Unexpected(val status: Int, val detail: String?) : ApiError
}

/** The result of one call. */
public sealed interface ApiResult<out T> {
    public data class Ok<T>(val value: T) : ApiResult<T>
    public data class Failed(val error: ApiError) : ApiResult<Nothing>
}

public object ErrorMapping {
    /** Maps a non-2xx answer. [isCreate] is true for `POST /api/plans`, the one call whose `409` means a duplicate. */
    public fun fromResponse(status: Int, body: String, isCreate: Boolean = false): ApiError {
        val problem = parseObject(body)
        val detail = problem?.string("detail")
        return when {
            status == 401 -> ApiError.Unauthorized
            status == 403 -> ApiError.Forbidden(detail ?: problem?.string("code"))
            status == 404 -> ApiError.NotFound
            status == 409 && isCreate -> ApiError.Duplicate
            status == 422 -> ApiError.Invalid(problems(problem, body))
            status >= 500 -> ApiError.Unreachable(ApiError.Unreachable.Reason.Server)
            else -> ApiError.Unexpected(status, detail ?: problem?.string("title"))
        }
    }

    /** Maps a call that got no answer. */
    public fun fromException(e: IOException): ApiError = ApiError.Unreachable(
        if (e is InterruptedIOException) ApiError.Unreachable.Reason.Timeout else ApiError.Unreachable.Reason.Network,
    )

    /**
     * The problems of a `422`, from the problem's `errors` member when it is a map of field to message(s), else from
     * `detail`, which the plan validator sends as every problem joined by `"; "` (service `controllers/plans.rs`). A
     * problem names a field when it starts with a wire field name followed by `:`, `[`, `" is "` or `" must "`.
     */
    internal fun problems(problem: JsonObject?, body: String): List<FieldProblem> {
        val fromErrors = (problem?.get("errors") as? JsonObject)?.flatMap { (field, value) ->
            messages(value).map { FieldProblem(field, it) }
        }.orEmpty()
        if (fromErrors.isNotEmpty()) return fromErrors
        val detail = problem?.string("detail") ?: problem?.string("title") ?: body.takeIf { problem == null && it.isNotBlank() }
        return detail?.split("; ")?.map { it.trim() }?.filter { it.isNotEmpty() }?.map { FieldProblem(fieldOf(it), it) }
            .orEmpty()
    }

    private val fieldPrefix = Regex("""^([a-z_]+)(\[\d+])?(:| is | must )""")

    internal fun fieldOf(message: String): String? = fieldPrefix.find(message)?.groupValues?.get(1)

    private fun messages(value: JsonElement): List<String> = when (value) {
        is JsonPrimitive -> listOfNotNull(value.contentOrNull)
        is JsonArray -> value.flatMap { messages(it) }
        is JsonObject -> listOfNotNull((value["message"] as? JsonPrimitive)?.contentOrNull)
    }

    private fun parseObject(body: String): JsonObject? =
        runCatching { WireJson.parseToJsonElement(body) as? JsonObject }.getOrNull()

    private fun JsonObject.string(key: String): String? = (get(key) as? JsonPrimitive)?.contentOrNull?.takeIf { it.isNotBlank() }
}
