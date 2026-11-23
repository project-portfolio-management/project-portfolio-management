// The screens' state and actions, apart from their views, so the behaviour is unit-tested on macOS (spec/mobile-apps.md
// MA-6). Each calls the service through the app model's client and hands a `401` to the app model.

import Foundation
import Observation
import PortfolioAPI
import PortfolioAuth

public enum Loadable<Value: Sendable & Equatable>: Sendable, Equatable {
    case loading
    case loaded(Value)
    case failed(ErrorPresentation)
}

// MARK: - Server (MA-1.1)

@MainActor @Observable
public final class ServerViewModel {
    public enum Check: Sendable, Equatable {
        case idle, checking, ready
        case failed(ErrorPresentation)
    }

    public var serviceURL: String
    public var issuer: String
    public var clientID: String
    public var redirectURI: String
    /// Problems by field (`serviceURL`, `issuer`, `clientID`, `redirectURI`), after a save or a check.
    public private(set) var errors: [String: String] = [:]
    public private(set) var check: Check = .idle
    @ObservationIgnored private let app: AppModel

    public static let fieldOrder = ["serviceURL", "issuer", "clientID", "redirectURI"]

    public init(app: AppModel) {
        self.app = app
        let settings = app.settings
        serviceURL = settings?.serviceURL.absoluteString ?? ""
        issuer = settings?.oidc?.issuer ?? ""
        clientID = settings?.oidc?.clientID ?? ""
        redirectURI = settings?.oidc?.redirectURI ?? OIDCSettings.defaultRedirectURI
    }

    /// The problems in field order, for the error summary.
    public var orderedErrors: [(field: String, message: String)] {
        Self.fieldOrder.compactMap { field in errors[field].map { (field, $0) } }
    }

    /// The settings as typed, or nil with `errors` filled.
    public func validated() -> ServerSettings? {
        var found: [String: String] = [:]
        var service: URL?
        switch ServerAddress.parse(serviceURL) {
        case .success(let url): service = url
        case .failure(.notAURL): found["serviceURL"] = Copy.serverErrorNotAURL.text
        case .failure(.needsHTTPS): found["serviceURL"] = Copy.serverErrorNeedsHTTPS.text
        }
        let issuerText = issuer.trimmingCharacters(in: .whitespacesAndNewlines)
        var oidc: OIDCSettings?
        if issuerText.isEmpty {
            if let service, !ServerAddress.isDevelopment(service) {
                found["issuer"] = Copy.serverErrorIssuerRequired.text
            }
        } else {
            switch ServerAddress.parse(issuerText) {
            case .success: break
            case .failure(.notAURL): found["issuer"] = Copy.serverErrorNotAURL.text
            case .failure(.needsHTTPS): found["issuer"] = Copy.serverErrorNeedsHTTPS.text
            }
            let client = clientID.trimmingCharacters(in: .whitespacesAndNewlines)
            if client.isEmpty { found["clientID"] = Copy.serverErrorClientID.text }
            let redirect = redirectURI.trimmingCharacters(in: .whitespacesAndNewlines)
            if URL(string: redirect)?.scheme == nil { found["redirectURI"] = Copy.serverErrorRedirectURI.text }
            oidc = OIDCSettings(issuer: issuerText, clientID: client, redirectURI: redirect)
        }
        errors = found
        guard found.isEmpty, let service else { return nil }
        return ServerSettings(serviceURL: service, oidc: oidc)
    }

    /// `GET /_health/ready` at the typed address.
    public func checkAddress() async {
        errors["serviceURL"] = nil
        let url: URL
        switch ServerAddress.parse(serviceURL) {
        case .success(let parsed): url = parsed
        case .failure(let problem):
            errors["serviceURL"] =
                problem == .notAURL ? Copy.serverErrorNotAURL.text : Copy.serverErrorNeedsHTTPS.text
            check = .idle
            return
        }
        check = .checking
        do {
            try await PortfolioClient(baseURL: url, transport: app.environment.transport).checkReady()
            check = .ready
        } catch {
            check = .failed(ErrorPresentation.of(error, subject: .list) ?? Self.unreachable)
        }
    }

    /// Validates and stores the settings; false when there are problems.
    @discardableResult
    public func save() -> Bool {
        guard let settings = validated() else { return false }
        app.save(settings)
        return true
    }

    static let unreachable = ErrorPresentation(
        heading: Copy.errorUnreachable.text, message: Copy.errorUnreachableDetail.text, canRetry: true)
}

// MARK: - Plans (MA-1.3)

@MainActor @Observable
public final class PlansViewModel {
    public var query = ""
    public private(set) var state: Loadable<[PlanRef]> = .loading
    /// The query the loaded rows answer ("" for the plain list).
    public private(set) var answeredQuery = ""
    @ObservationIgnored private let app: AppModel

    public init(app: AppModel) {
        self.app = app
    }

    /// The active plans, or the search's matches when the box is not empty.
    public func load() async {
        guard let client = app.client else { return }
        let q = query.trimmingCharacters(in: .whitespacesAndNewlines)
        if case .loaded = state {} else { state = .loading }
        do {
            let rows = q.isEmpty ? try await client.listPlans() : try await client.searchPlans(q)
            answeredQuery = q
            state = .loaded(rows)
        } catch {
            if app.handle(error) { return }
            state = .failed(ErrorPresentation.of(error, subject: .list) ?? ServerViewModel.unreachable)
        }
    }
}

// MARK: - Plan (MA-1.4, MA-1.5)

@MainActor @Observable
public final class PlanViewModel {
    public struct Content: Sendable, Equatable {
        public var plan: Plan
        public var children: [PlanRef]
        public var tasks: [PlanTask]
        public var truncated: Bool
    }

    public let ref: PlanRef
    public private(set) var state: Loadable<Content> = .loading
    /// The task being moved, while its move runs.
    public private(set) var moving: String?
    /// Why the last move failed; the task kept its status.
    public var moveError: ErrorPresentation?
    @ObservationIgnored private let app: AppModel

    public init(ref: PlanRef, app: AppModel) {
        self.ref = ref
        self.app = app
    }

    public func load() async {
        guard let client = app.client else { return }
        if case .loaded = state {} else { state = .loading }
        do {
            async let plan = client.plan(ref.pid)
            async let children = client.listPlans(parent: ref.pid)
            async let tasks = client.tasks(planPID: ref.pid)
            let list = try await tasks
            state = .loaded(
                Content(plan: try await plan, children: try await children, tasks: list.tasks, truncated: list.truncated))
        } catch let error as APIError {
            if app.handle(error) { return }
            state = .failed(ErrorPresentation.of(error, subject: .plan) ?? ServerViewModel.unreachable)
        } catch {
            state = .failed(ServerViewModel.unreachable)
        }
    }

    /// Moves a task (MA-1.5). The task changes status only when the service says it moved; a failed move leaves it as it
    /// was (MA-3). Returns whether it moved.
    @discardableResult
    public func move(_ task: PlanTask, to status: TaskStatus) async -> Bool {
        guard let client = app.client, case .loaded(var content) = state, moving == nil else { return false }
        if task.status == status { return true }
        moving = task.pid
        moveError = nil
        defer { moving = nil }
        do {
            let moved = try await client.moveTask(planPID: ref.pid, taskPID: task.pid, to: status)
            if case .loaded(let latest) = state { content = latest }
            if let index = content.tasks.firstIndex(where: { $0.pid == moved.pid }) { content.tasks[index] = moved }
            state = .loaded(content)
            return true
        } catch {
            if app.handle(error) { return false }
            moveError = ErrorPresentation.of(error, subject: .task) ?? ServerViewModel.unreachable
            return false
        }
    }
}

// MARK: - New plan (MA-1.6)

@MainActor @Observable
public final class NewPlanViewModel {
    public var name = ""
    public var kind: PlanKind?
    public var status: PlanStatus?
    public var code = ""
    public var hasStart = false
    public var start = Date()
    public var hasTarget = false
    public var target = Date()
    /// The `422` problems, in the service's order.
    public private(set) var problems: [Problem] = []
    /// Any other failure (`409`, `403`, unreachable).
    public var failure: ErrorPresentation?
    public private(set) var saving = false
    @ObservationIgnored private let app: AppModel

    public init(app: AppModel) {
        self.app = app
    }

    /// The body the form sends; an empty field is left out.
    public func body(calendar: Calendar = .current) -> NewPlan {
        let trimmedCode = code.trimmingCharacters(in: .whitespacesAndNewlines)
        return NewPlan(
            name: name.trimmingCharacters(in: .whitespacesAndNewlines), kind: kind, status: status,
            code: trimmedCode.isEmpty ? nil : trimmedCode,
            startDate: hasStart ? CalendarDate(pickedDate: start, calendar: calendar) : nil,
            targetDate: hasTarget ? CalendarDate(pickedDate: target, calendar: calendar) : nil)
    }

    /// The problems for one field (its wire name), joined, for the field's error line.
    public func error(for field: String) -> String? {
        let messages = problems.filter { $0.field == field }.map(\.message)
        return messages.isEmpty ? nil : messages.joined(separator: "\n")
    }

    /// `POST /api/plans`. Returns the new plan's reference, or nil with `problems` or `failure` set.
    public func submit() async -> PlanRef? {
        guard let client = app.client, !saving else { return nil }
        saving = true
        problems = []
        failure = nil
        defer { saving = false }
        do {
            return try await client.createPlan(body())
        } catch {
            if app.handle(error) { return nil }
            if case .validation(let found) = error, !found.isEmpty {
                problems = found
            } else {
                failure = ErrorPresentation.of(error, subject: .create) ?? ServerViewModel.unreachable
            }
            return nil
        }
    }
}
