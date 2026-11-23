// The stub's starting data: two top-level plans, one child plan with a custom status, and five tasks, one in each status.
// Each record carries `from_a_newer_service`, a field the app does not know.

import Foundation

public enum StubFixtures {
    public static let harbourPID = "00000000-0000-4000-8000-000000000001"
    public static let quayPID = "00000000-0000-4000-8000-000000000002"
    public static let libraryPID = "00000000-0000-4000-8000-000000000003"
    public static let surveyTaskPID = "10000000-0000-4000-8000-000000000001"

    static var plans: [StubService.StoredPlan] {
        [
            .init(
                pid: harbourPID,
                json: [
                    "name": "Harbour redevelopment", "kind": "Program", "status": "Active", "code": "HRB-2026",
                    "owner_org_name": "Port of Aberford", "start_date": "2026-01-12", "target_date": "2027-06-30",
                    "keywords": ["harbour", "regeneration"], "goals": [["title": "Reopen the north quay"]],
                    "from_a_newer_service": ["anything": true],
                ]),
            .init(
                pid: quayPID,
                json: [
                    "name": "Quay wall survey", "kind": "Project", "status": ["Custom": "Awaiting funding"],
                    "parent_ref": harbourPID, "start_date": "2026-03-02", "from_a_newer_service": 1,
                ]),
            .init(
                pid: libraryPID,
                json: ["name": "Library digitisation", "kind": "Project", "status": "Proposed", "from_a_newer_service": "x"]),
        ]
    }

    static var tasks: [String: [[String: Any]]] {
        func task(_ n: Int, _ title: String, _ status: String, due: String? = nil, points: Int? = nil, blocked: Int? = nil)
            -> [String: Any]
        {
            [
                "pid": "10000000-0000-4000-8000-\(String(format: "%012d", n))", "title": title, "description": NSNull(),
                "status": status, "assignee_ref": NSNull(), "points": points.map { $0 as Any } ?? NSNull(),
                "due_date": due.map { $0 as Any } ?? NSNull(), "blocked_days": blocked.map { $0 as Any } ?? NSNull(),
                "sprint_pid": NSNull(), "created_at": "2026-01-12T09:00:00Z", "status_changed_at": "2026-01-12T09:00:00Z",
                "done_at": NSNull(), "from_a_newer_service": true,
            ]
        }
        return [
            harbourPID: [
                task(1, "Commission the diver survey", "todo", due: "2026-12-04", points: 3),
                task(2, "Draft the tender", "in_progress", due: "2027-01-15", points: 5),
                task(3, "Review the ecology report", "in_review"),
                task(4, "Agree access with the port authority", "blocked", blocked: 4),
                task(5, "Appoint the project lead", "done", points: 1),
            ],
            quayPID: [],
            libraryPID: [],
        ]
    }
}
