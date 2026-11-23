// The words for the service's values, and what the app says for each error (spec/mobile-apps.md MA-3). Pure, so each row
// of MA-3 has a unit test.

import Foundation
import PortfolioAPI
import PortfolioAuth

extension PlanKind {
    public var label: String {
        switch self {
        case .portfolio: Copy.kindPortfolio.text
        case .project: Copy.kindProject.text
        case .product: Copy.kindProduct.text
        case .program: Copy.kindProgram.text
        case .practice: Copy.kindPractice.text
        case .process: Copy.kindProcess.text
        case .purpose: Copy.kindPurpose.text
        case .pathway: Copy.kindPathway.text
        case .proposal: Copy.kindProposal.text
        case .other(let raw): raw
        }
    }
}

extension PlanStatus {
    /// A custom status is shown by its text (MA-1).
    public var label: String {
        switch self {
        case .proposed: Copy.planStatusProposed.text
        case .active: Copy.planStatusActive.text
        case .onHold: Copy.planStatusOnHold.text
        case .completed: Copy.planStatusCompleted.text
        case .cancelled: Copy.planStatusCancelled.text
        case .custom(let text), .other(let text): text
        }
    }
}

extension TaskStatus {
    public var label: String {
        switch self {
        case .todo: Copy.taskStatusTodo.text
        case .inProgress: Copy.taskStatusInProgress.text
        case .inReview: Copy.taskStatusInReview.text
        case .done: Copy.taskStatusDone.text
        case .blocked: Copy.taskStatusBlocked.text
        case .other(let raw): raw
        }
    }
}

/// What a screen shows for a failed call: a heading, the message, and whether a **Try again** is offered.
public struct ErrorPresentation: Sendable, Hashable {
    public let heading: String
    public let message: String
    public let canRetry: Bool

    /// What the failed call was about, which decides the words for a `404`.
    public enum Subject: Sendable {
        case plan, task, create, list
    }

    /// The presentation of `error`, or nil for a `401`, which signs the person out instead of showing an error.
    public static func of(_ error: APIError, subject: Subject) -> ErrorPresentation? {
        let heading = Copy.errorHeading.text
        switch error {
        case .unauthorized:
            return nil
        case .forbidden(let rule):
            let message = [Copy.errorForbidden.text, rule.map { Copy.errorForbiddenRule($0) }]
                .compactMap { $0 }.joined(separator: "\n")
            return .init(heading: heading, message: message, canRetry: false)
        case .notFound:
            let message = subject == .task ? Copy.errorNotFoundTask.text : Copy.errorNotFoundPlan.text
            return .init(heading: heading, message: message, canRetry: false)
        case .conflict:
            return .init(heading: heading, message: Copy.errorConflict.text, canRetry: false)
        case .validation(let problems):
            // On a form the problems go to the ErrorSummary; elsewhere (a move refused by a WIP limit or the workflow) the
            // service's reasons are the message.
            let heading = subject == .task ? Copy.moveFailed.text : Copy.errorSummaryTitle.text
            return .init(heading: heading, message: problems.map(\.message).joined(separator: "\n"), canRetry: false)
        case .unreachable:
            return .init(
                heading: Copy.errorUnreachable.text, message: Copy.errorUnreachableDetail.text, canRetry: true)
        case .unexpected(let status, _):
            return .init(heading: heading, message: Copy.errorUnexpected(status), canRetry: true)
        }
    }

    /// The message for a sign-in that did not finish.
    public static func of(_ error: AuthError) -> String {
        switch error {
        case .issuerNotAllowed, .endpointNotAllowed: Copy.signInErrorNotAllowed.text
        case .discoveryFailed: Copy.signInErrorDiscovery.text
        case .issuerMismatch(let found): Copy.signInErrorIssuerMismatch(found)
        case .s256Unsupported: Copy.signInErrorS256.text
        case .stateMismatch: Copy.signInErrorState.text
        case .provider(let code, _): Copy.signInErrorProvider(code)
        case .missingCode, .tokenRequestFailed: Copy.signInErrorToken.text
        case .cancelled: Copy.signInErrorCancelled.text
        case .signedOut: Copy.signInSessionEnded.text
        case .invalidRedirectURI: Copy.signInErrorRedirect.text
        }
    }
}
