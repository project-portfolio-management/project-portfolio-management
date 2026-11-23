// The service's wire shapes that the first slice reads and writes (spec/mobile-apps.md MA-1): `Plan` (the matcher type),
// `PlanRef`, the task row, and the two request bodies. A field the app does not know is ignored, so a newer service does not
// break it; an enum value the app does not know is kept by its text rather than refused.

import Foundation

/// A plan's lifecycle status: a unit string, or `{"Custom": "…"}`.
public enum PlanStatus: Sendable, Hashable, Codable {
    case proposed, active, onHold, completed, cancelled
    /// `{"Custom": text}`: shown by its text.
    case custom(String)
    /// A unit value this app does not know (a newer service): shown by its text, sent back as it came.
    case other(String)

    /// The unit statuses the create form offers.
    public static let units: [PlanStatus] = [.proposed, .active, .onHold, .completed, .cancelled]

    var unitName: String? {
        switch self {
        case .proposed: "Proposed"
        case .active: "Active"
        case .onHold: "OnHold"
        case .completed: "Completed"
        case .cancelled: "Cancelled"
        case .other(let raw): raw
        case .custom: nil
        }
    }

    public init(from decoder: any Decoder) throws {
        let container = try decoder.singleValueContainer()
        if let raw = try? container.decode(String.self) {
            switch raw {
            case "Proposed": self = .proposed
            case "Active": self = .active
            case "OnHold": self = .onHold
            case "Completed": self = .completed
            case "Cancelled": self = .cancelled
            default: self = .other(raw)
            }
            return
        }
        let custom = try container.decode([String: String].self)
        guard let text = custom["Custom"] else {
            throw DecodingError.dataCorruptedError(
                in: container, debugDescription: "a plan status is a string or {\"Custom\": …}")
        }
        self = .custom(text)
    }

    public func encode(to encoder: any Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .custom(let text): try container.encode(["Custom": text])
        default: try container.encode(unitName ?? "")
        }
    }
}

/// A plan's optional descriptive label. A label this app does not know is kept by its text.
public enum PlanKind: Sendable, Hashable, Codable {
    case portfolio, project, product, program, practice, process, purpose, pathway, proposal
    case other(String)

    public static let all: [PlanKind] = [
        .portfolio, .project, .product, .program, .practice, .process, .purpose, .pathway, .proposal,
    ]

    /// The wire name (`Portfolio`, …).
    public var wireName: String {
        switch self {
        case .portfolio: "Portfolio"
        case .project: "Project"
        case .product: "Product"
        case .program: "Program"
        case .practice: "Practice"
        case .process: "Process"
        case .purpose: "Purpose"
        case .pathway: "Pathway"
        case .proposal: "Proposal"
        case .other(let raw): raw
        }
    }

    public init(wireName: String) {
        self = Self.all.first { $0.wireName == wireName } ?? .other(wireName)
    }

    public init(from decoder: any Decoder) throws {
        self.init(wireName: try decoder.singleValueContainer().decode(String.self))
    }

    public func encode(to encoder: any Encoder) throws {
        var container = encoder.singleValueContainer()
        try container.encode(wireName)
    }
}

/// An ISO calendar date (`YYYY-MM-DD`): a day, not an instant, so it is never converted through a time zone.
public struct CalendarDate: Sendable, Hashable, Comparable, Codable, CustomStringConvertible {
    public let year: Int
    public let month: Int
    public let day: Int

    /// The calendar used to check and show a date: Gregorian at UTC, so a day never moves to its neighbour.
    static let fixedCalendar: Calendar = {
        var calendar = Calendar(identifier: .gregorian)
        calendar.timeZone = TimeZone(identifier: "UTC")!
        return calendar
    }()

    /// `nil` unless `year-month-day` is a real date.
    public init?(year: Int, month: Int, day: Int) {
        let components = DateComponents(year: year, month: month, day: day)
        guard (1...9999).contains(year), components.isValidDate(in: Self.fixedCalendar) else { return nil }
        self.year = year
        self.month = month
        self.day = day
    }

    /// Parses `YYYY-MM-DD` exactly (four, two and two digits).
    public init?(iso: String) {
        let parts = iso.split(separator: "-", omittingEmptySubsequences: false)
        guard parts.count == 3, parts[0].count == 4, parts[1].count == 2, parts[2].count == 2,
            parts.allSatisfy({ $0.allSatisfy(\.isASCII) && $0.allSatisfy(\.isNumber) }),
            let y = Int(parts[0]), let m = Int(parts[1]), let d = Int(parts[2])
        else { return nil }
        self.init(year: y, month: m, day: d)
    }

    /// The day a person picked on a date picker, read in the calendar and zone the picker showed it in.
    public init(pickedDate date: Date, calendar: Calendar = .current) {
        let c = calendar.dateComponents([.year, .month, .day], from: date)
        self.year = c.year ?? 1970
        self.month = c.month ?? 1
        self.day = c.day ?? 1
    }

    public var iso: String { String(format: "%04d-%02d-%02d", year, month, day) }
    public var description: String { iso }

    /// The date as the person's locale writes it, for example "12 Jan 2026" or "Jan 12, 2026".
    public func formatted(locale: Locale = .current) -> String {
        let date = Self.fixedCalendar.date(from: DateComponents(year: year, month: month, day: day))!
        var style = Date.FormatStyle(date: .abbreviated, time: .omitted, locale: locale, calendar: Self.fixedCalendar)
        style.timeZone = Self.fixedCalendar.timeZone
        return date.formatted(style)
    }

    public static func < (lhs: Self, rhs: Self) -> Bool {
        (lhs.year, lhs.month, lhs.day) < (rhs.year, rhs.month, rhs.day)
    }

    public init(from decoder: any Decoder) throws {
        let container = try decoder.singleValueContainer()
        let raw = try container.decode(String.self)
        guard let date = CalendarDate(iso: raw) else {
            throw DecodingError.dataCorruptedError(in: container, debugDescription: "not a YYYY-MM-DD date: \(raw)")
        }
        self = date
    }

    public func encode(to encoder: any Encoder) throws {
        var container = encoder.singleValueContainer()
        try container.encode(iso)
    }
}

/// `{pid, name}`: what the list, search and create answer.
public struct PlanRef: Sendable, Hashable, Codable, Identifiable {
    public let pid: String
    public let name: String
    public var id: String { pid }

    public init(pid: String, name: String) {
        self.pid = pid
        self.name = name
    }
}

/// The stored plan (`GET /api/plans/{pid}`), as far as the first slice shows it. Goals, identifiers, relationships and the
/// other fields the matcher carries are ignored on read.
public struct Plan: Sendable, Hashable, Decodable {
    public let name: String
    public let kind: PlanKind?
    public let status: PlanStatus?
    public let code: String?
    public let ownerOrgName: String?
    public let leadRef: String?
    public let parentRef: String?
    public let startDate: CalendarDate?
    public let targetDate: CalendarDate?
    public let alternateNames: [String]
    public let keywords: [String]
    public let tags: [String]

    enum CodingKeys: String, CodingKey {
        case name, kind, status, code, keywords, tags
        case ownerOrgName = "owner_org_name"
        case leadRef = "lead_ref"
        case parentRef = "parent_ref"
        case startDate = "start_date"
        case targetDate = "target_date"
        case alternateNames = "alternate_names"
    }

    public init(from decoder: any Decoder) throws {
        let c = try decoder.container(keyedBy: CodingKeys.self)
        name = try c.decode(String.self, forKey: .name)
        kind = try c.decodeIfPresent(PlanKind.self, forKey: .kind)
        status = try c.decodeIfPresent(PlanStatus.self, forKey: .status)
        code = try c.decodeIfPresent(String.self, forKey: .code)
        ownerOrgName = try c.decodeIfPresent(String.self, forKey: .ownerOrgName)
        leadRef = try c.decodeIfPresent(String.self, forKey: .leadRef)
        parentRef = try c.decodeIfPresent(String.self, forKey: .parentRef)
        startDate = try c.decodeIfPresent(CalendarDate.self, forKey: .startDate)
        targetDate = try c.decodeIfPresent(CalendarDate.self, forKey: .targetDate)
        alternateNames = try c.decodeIfPresent([String].self, forKey: .alternateNames) ?? []
        keywords = try c.decodeIfPresent([String].self, forKey: .keywords) ?? []
        tags = try c.decodeIfPresent([String].self, forKey: .tags) ?? []
    }
}

/// A task's board status. The service's built-in workflow has five; a plan's own workflow may name others, which are kept by
/// their text.
public enum TaskStatus: Sendable, Hashable, Codable, Identifiable {
    case todo, inProgress, inReview, done, blocked
    case other(String)

    /// The statuses a task can be moved to from the app.
    public static let board: [TaskStatus] = [.todo, .inProgress, .inReview, .done, .blocked]

    public var wireName: String {
        switch self {
        case .todo: "todo"
        case .inProgress: "in_progress"
        case .inReview: "in_review"
        case .done: "done"
        case .blocked: "blocked"
        case .other(let raw): raw
        }
    }

    public var id: String { wireName }

    public init(wireName: String) {
        self = Self.board.first { $0.wireName == wireName } ?? .other(wireName)
    }

    public init(from decoder: any Decoder) throws {
        self.init(wireName: try decoder.singleValueContainer().decode(String.self))
    }

    public func encode(to encoder: any Encoder) throws {
        var container = encoder.singleValueContainer()
        try container.encode(wireName)
    }
}

/// One task row, as `GET /api/plans/{pid}/tasks` lists it and the move answers it.
public struct PlanTask: Sendable, Hashable, Decodable, Identifiable {
    public let pid: String
    public let title: String
    public let description: String?
    public let status: TaskStatus
    public let assigneeRef: String?
    public let points: Int?
    public let dueDate: CalendarDate?
    public let blockedDays: Int?
    public var id: String { pid }

    enum CodingKeys: String, CodingKey {
        case pid, title, description, status, points
        case assigneeRef = "assignee_ref"
        case dueDate = "due_date"
        case blockedDays = "blocked_days"
    }

    public init(
        pid: String, title: String, description: String? = nil, status: TaskStatus, assigneeRef: String? = nil,
        points: Int? = nil, dueDate: CalendarDate? = nil, blockedDays: Int? = nil
    ) {
        self.pid = pid
        self.title = title
        self.description = description
        self.status = status
        self.assigneeRef = assigneeRef
        self.points = points
        self.dueDate = dueDate
        self.blockedDays = blockedDays
    }
}

/// `GET /api/plans/{pid}/tasks`: an object, not a bare array (`counts` is not read).
public struct TaskList: Sendable, Hashable, Decodable {
    public let tasks: [PlanTask]
    public let total: Int?
    /// More tasks follow than this page holds (the service caps a page at 2,000).
    public let truncated: Bool

    enum CodingKeys: String, CodingKey { case tasks, total, truncated }

    public init(from decoder: any Decoder) throws {
        let c = try decoder.container(keyedBy: CodingKeys.self)
        tasks = try c.decode([PlanTask].self, forKey: .tasks)
        total = try c.decodeIfPresent(Int.self, forKey: .total)
        truncated = try c.decodeIfPresent(Bool.self, forKey: .truncated) ?? false
    }
}

/// The body of `POST /api/plans` from the new-plan screen (MA-1.6). A field left empty is left out.
public struct NewPlan: Sendable, Hashable, Encodable {
    public var name: String
    public var kind: PlanKind?
    public var status: PlanStatus?
    public var code: String?
    public var startDate: CalendarDate?
    public var targetDate: CalendarDate?

    public init(
        name: String, kind: PlanKind? = nil, status: PlanStatus? = nil, code: String? = nil,
        startDate: CalendarDate? = nil, targetDate: CalendarDate? = nil
    ) {
        self.name = name
        self.kind = kind
        self.status = status
        self.code = code
        self.startDate = startDate
        self.targetDate = targetDate
    }

    enum CodingKeys: String, CodingKey {
        case name, kind, status, code
        case startDate = "start_date"
        case targetDate = "target_date"
    }

    public func encode(to encoder: any Encoder) throws {
        var c = encoder.container(keyedBy: CodingKeys.self)
        try c.encode(name, forKey: .name)
        try c.encodeIfPresent(kind, forKey: .kind)
        try c.encodeIfPresent(status, forKey: .status)
        try c.encodeIfPresent(code, forKey: .code)
        try c.encodeIfPresent(startDate, forKey: .startDate)
        try c.encodeIfPresent(targetDate, forKey: .targetDate)
    }
}

/// The body of the move (`PATCH /api/plans/{pid}/tasks/{tid}`).
struct MoveTaskBody: Encodable {
    let status: TaskStatus
}
