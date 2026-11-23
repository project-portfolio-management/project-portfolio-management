// What an answer that is not a success means to the app (spec/mobile-apps.md MA-3). The service answers errors as RFC 9457
// problem details (`application/problem+json`: `title`, `status`, `detail`, `code`, and for some validation failures an
// `errors` member of field → messages); a plan validation failure lists every problem in `detail`, joined with "; ".

import Foundation

/// One problem the service names in a `422`, with the field it names where it names one.
public struct Problem: Sendable, Hashable {
    /// The wire name of the field (`name`, `start_date`, …), or nil when the problem names none.
    public let field: String?
    /// The problem, in the service's words.
    public let message: String

    public init(field: String?, message: String) {
        self.field = field
        self.message = message
    }
}

public enum APIError: Error, Sendable, Hashable {
    /// The reason the service could not be reached.
    public enum Unreachable: Sendable, Hashable {
        case offline
        case timeout
        case server(status: Int)
    }

    /// `401`: the person must sign in again.
    case unauthorized
    /// `403`: the rule the service names in the body, when it names one.
    case forbidden(rule: String?)
    /// `404`: the plan or task is not there.
    case notFound
    /// `409`: a plan like this one already exists.
    case conflict(detail: String?)
    /// `422`: every problem the service lists.
    case validation([Problem])
    /// No network, a timeout, or a `5xx`.
    case unreachable(Unreachable)
    /// `400`, another status the first slice does not expect, or a success whose body is not the shape the app reads.
    case unexpected(status: Int, detail: String?)
}

/// The members of a problem-details body the app reads.
struct ProblemDetails: Decodable {
    let title: String?
    let detail: String?
    let code: String?
    let errors: [String: [String]]?

    enum CodingKeys: String, CodingKey { case title, detail, code, errors }

    init(from decoder: any Decoder) throws {
        let c = try decoder.container(keyedBy: CodingKeys.self)
        title = try? c.decodeIfPresent(String.self, forKey: .title)
        detail = try? c.decodeIfPresent(String.self, forKey: .detail)
        code = try? c.decodeIfPresent(String.self, forKey: .code)
        // `errors` is field → [message]; a field with one message as a bare string is read too; any other shape is ignored.
        if let many = try? c.decodeIfPresent([String: [String]].self, forKey: .errors) {
            errors = many
        } else if let one = try? c.decodeIfPresent([String: String].self, forKey: .errors) {
            errors = one.mapValues { [$0] }
        } else {
            errors = nil
        }
    }

    static func read(_ body: Data) -> ProblemDetails? {
        try? JSONDecoder().decode(ProblemDetails.self, from: body)
    }
}

extension APIError {
    /// The fields a plan problem may name; a problem that begins with one of these words is shown beside that field.
    static let planFields: Set<String> = [
        "name", "kind", "status", "code", "start_date", "target_date", "parent_ref", "alternate_names", "goals",
        "identifiers", "in_language", "keywords", "tags", "owner_org_id", "owner_org_name", "lead_ref",
    ]

    /// The error an answer stands for, or nil for a success (`2xx`).
    public static func from(_ response: HTTPResponse) -> APIError? {
        let status = response.status
        if (200..<300).contains(status) { return nil }
        let problem = ProblemDetails.read(response.body)
        switch status {
        case 401: return .unauthorized
        case 403: return .forbidden(rule: nonBlank(problem?.detail))
        case 404: return .notFound
        case 409: return .conflict(detail: nonBlank(problem?.detail))
        case 422: return .validation(problems(from: problem))
        case 500..<600: return .unreachable(.server(status: status))
        default: return .unexpected(status: status, detail: nonBlank(problem?.detail))
        }
    }

    /// Every problem in a `422`: the `errors` member when there is one, else each "; "-separated part of `detail`.
    static func problems(from problem: ProblemDetails?) -> [Problem] {
        if let errors = problem?.errors, !errors.isEmpty {
            return errors.keys.sorted().flatMap { field in
                errors[field, default: []].map { Problem(field: field, message: "\(field) \($0)") }
            }
        }
        guard let detail = nonBlank(problem?.detail) else { return [] }
        return detail.components(separatedBy: "; ").compactMap { part in
            let message = part.trimmingCharacters(in: .whitespaces)
            return message.isEmpty ? nil : Problem(field: field(named: message), message: message)
        }
    }

    /// The field a problem sentence names: `goals[0]: …` names `goals`; `start_date must …` names `start_date`.
    static func field(named message: String) -> String? {
        let head = message.prefix { $0.isLetter || $0 == "_" }
        let word = String(head)
        guard planFields.contains(word) else { return nil }
        let rest = message.dropFirst(head.count)
        guard rest.isEmpty || rest.first == " " || rest.first == ":" || rest.first == "[" else { return nil }
        return word
    }

    /// The error for a transport failure (no answer at all).
    public static func from(transportError error: any Error) -> APIError {
        if let urlError = error as? URLError, urlError.code == .timedOut { return .unreachable(.timeout) }
        return .unreachable(.offline)
    }

    private static func nonBlank(_ text: String?) -> String? {
        guard let text = text?.trimmingCharacters(in: .whitespacesAndNewlines), !text.isEmpty else { return nil }
        return text
    }
}
