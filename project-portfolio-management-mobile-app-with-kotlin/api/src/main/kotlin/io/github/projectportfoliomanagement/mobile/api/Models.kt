// The service's wire shapes, as far as the first slice reads them (../spec/mobile-apps.md MA-1). Every class ignores a field
// it does not know (see [WireJson]), so a newer service does not break the app.
package io.github.projectportfoliomanagement.mobile.api

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.buildClassSerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonDecoder
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonEncoder
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.contentOrNull
import kotlinx.serialization.json.jsonPrimitive

/** The one JSON configuration for the service's bodies: unknown fields are ignored, nulls are left out of requests. */
public val WireJson: Json = Json {
    ignoreUnknownKeys = true
    explicitNulls = false
    coerceInputValues = true
}

/** `{pid, name}`, as `GET /api/plans`, the search and `POST /api/plans` answer. */
@Serializable
public data class PlanRef(val pid: String, val name: String)

/**
 * A plan's lifecycle status. The service sends a unit variant as a bare string (`"Active"`) and a custom one as
 * `{"Custom": "…"}`. A unit string the app does not know (a newer service's) is kept as [Named] and shown by its text.
 */
@Serializable(with = PlanStatusSerializer::class)
public sealed class PlanStatus {
    /** A unit variant: `Proposed`, `Active`, `OnHold`, `Completed`, `Cancelled`, or one a newer service adds. */
    public data class Named(val name: String) : PlanStatus()

    /** `{"Custom": text}`: shown by its text. */
    public data class Custom(val text: String) : PlanStatus()

    public companion object {
        public val Proposed: PlanStatus = Named("Proposed")
        public val Active: PlanStatus = Named("Active")
        public val OnHold: PlanStatus = Named("OnHold")
        public val Completed: PlanStatus = Named("Completed")
        public val Cancelled: PlanStatus = Named("Cancelled")

        /** The unit statuses a new plan may be given, in the operator UI's order. */
        public val units: List<PlanStatus> = listOf(Proposed, Active, OnHold, Completed, Cancelled)
    }
}

/** Reads a bare string or `{"Custom": "…"}`; writes the same shapes. */
public object PlanStatusSerializer : KSerializer<PlanStatus> {
    override val descriptor: SerialDescriptor = buildClassSerialDescriptor("PlanStatus")

    override fun deserialize(decoder: Decoder): PlanStatus {
        val json = decoder as? JsonDecoder ?: throw SerializationException("PlanStatus is read from JSON only")
        return when (val element: JsonElement = json.decodeJsonElement()) {
            is JsonPrimitive -> element.contentOrNull?.let { PlanStatus.Named(it) }
                ?: throw SerializationException("PlanStatus is null")
            is JsonObject -> {
                val custom = element["Custom"]?.jsonPrimitive?.contentOrNull
                    ?: throw SerializationException("PlanStatus object without a Custom string: $element")
                PlanStatus.Custom(custom)
            }
            else -> throw SerializationException("PlanStatus is neither a string nor an object: $element")
        }
    }

    override fun serialize(encoder: Encoder, value: PlanStatus) {
        val json = encoder as? JsonEncoder ?: throw SerializationException("PlanStatus is written as JSON only")
        json.encodeJsonElement(
            when (value) {
                is PlanStatus.Named -> JsonPrimitive(value.name)
                is PlanStatus.Custom -> JsonObject(mapOf("Custom" to JsonPrimitive(value.text)))
            },
        )
    }
}

/** The plan kinds the service names (an optional descriptive label). Kept as strings so a newer kind still reads. */
public object PlanKinds {
    public val all: List<String> =
        listOf("Portfolio", "Project", "Product", "Program", "Practice", "Process", "Purpose", "Pathway", "Proposal")
}

/** The stored `Plan` (the matcher type), as far as the app shows it. Every field but `name` may be absent. */
@Serializable
public data class Plan(
    val name: String,
    val kind: String? = null,
    val code: String? = null,
    val status: PlanStatus? = null,
    @SerialName("owner_org_name") val ownerOrgName: String? = null,
    @SerialName("parent_ref") val parentRef: String? = null,
    @SerialName("start_date") val startDate: String? = null,
    @SerialName("target_date") val targetDate: String? = null,
    val keywords: List<String> = emptyList(),
    val tags: List<String> = emptyList(),
)

/** `POST /api/plans`: the fields the first slice's form sets (MA-1.6). Absent fields are left out of the body. */
@Serializable
public data class NewPlan(
    val name: String,
    val kind: String? = null,
    val status: PlanStatus? = null,
    val code: String? = null,
    @SerialName("start_date") val startDate: String? = null,
    @SerialName("target_date") val targetDate: String? = null,
)

/** The built-in task statuses, in board order. A plan's workflow may declare others; a task keeps whatever it is sent. */
public object TaskStatuses {
    public const val TODO: String = "todo"
    public const val IN_PROGRESS: String = "in_progress"
    public const val IN_REVIEW: String = "in_review"
    public const val DONE: String = "done"
    public const val BLOCKED: String = "blocked"
    public val all: List<String> = listOf(TODO, IN_PROGRESS, IN_REVIEW, DONE, BLOCKED)
}

/** One task row (`GET /api/plans/{pid}/tasks`, and the answer to a move). */
@Serializable
public data class Task(
    val pid: String,
    val title: String,
    val description: String? = null,
    val status: String,
    @SerialName("assignee_ref") val assigneeRef: String? = null,
    val points: Int? = null,
    @SerialName("due_date") val dueDate: String? = null,
    @SerialName("blocked_days") val blockedDays: Int? = null,
)

/** `PATCH /api/plans/{pid}/tasks/{tid}`. */
@Serializable
public data class TaskMove(val status: String)
