// MA-1.6: a new plan, `POST /api/plans`. The service checks the fields; every problem it lists in a `422` is shown in Lily's
// `ErrorSummary` and beside the field it names (MA-3).

import LilyUI
import PortfolioAPI
import SwiftUI

struct NewPlanScreen: View {
    @State private var model: NewPlanViewModel
    @FocusState private var focused: String?
    @Environment(\.dismiss) private var dismiss
    let created: (PlanRef) -> Void

    init(app: AppModel, created: @escaping (PlanRef) -> Void) {
        _model = State(initialValue: NewPlanViewModel(app: app))
        self.created = created
    }

    var body: some View {
        NavigationStack {
            ScrollViewReader { scroller in
            Form {
                if !model.problems.isEmpty {
                    Section {
                        ErrorSummary(
                            title: Copy.errorSummaryTitle.text,
                            problems: model.problems.map { problem in
                                .init(problem.message) { focused = problem.field }
                            })
                    }
                    .id("summary")
                }
                if let failure = model.failure {
                    Section { LilyAlert(kind: .error, heading: failure.heading, message: failure.message) }
                }
                Section {
                    Field(
                        label: Copy.newPlanName.text, error: model.error(for: "name"), required: true,
                        requiredLabel: Copy.fieldRequired.text
                    ) {
                        TextField("", text: $model.name)
                            .fieldBox()
                            .focused($focused, equals: "name")
                            .accessibilityIdentifier("new-plan-name")
                    }
                    Picker(selection: $model.kind) {
                        Text(verbatim: Copy.newPlanNone.text).tag(PlanKind?.none)
                        ForEach(PlanKind.all, id: \.wireName) { kind in
                            Text(verbatim: kind.label).tag(PlanKind?.some(kind))
                        }
                    } label: {
                        Text(verbatim: Copy.newPlanKind.text)
                    }
                    .frame(minHeight: 44)
                    .accessibilityIdentifier("new-plan-kind")
                    if let error = model.error(for: "kind") { Text(verbatim: error).bold() }
                    Picker(selection: $model.status) {
                        Text(verbatim: Copy.newPlanNone.text).tag(PlanStatus?.none)
                        ForEach(PlanStatus.units, id: \.label) { status in
                            Text(verbatim: status.label).tag(PlanStatus?.some(status))
                        }
                    } label: {
                        Text(verbatim: Copy.newPlanStatus.text)
                    }
                    .frame(minHeight: 44)
                    .accessibilityIdentifier("new-plan-status")
                    if let error = model.error(for: "status") { Text(verbatim: error).bold() }
                    Field(label: Copy.newPlanCode.text, description: Copy.newPlanCodeHint.text, error: model.error(for: "code")) {
                        TextField("", text: $model.code)
                            .plainEntry().fieldBox()
                            .focused($focused, equals: "code")
                            .accessibilityIdentifier("new-plan-code")
                    }
                }
                Section {
                    Toggle(isOn: $model.hasStart) { Text(verbatim: Copy.newPlanHasStart.text) }
                        .accessibilityIdentifier("new-plan-has-start")
                    if model.hasStart {
                        DatePicker(selection: $model.start, displayedComponents: .date) {
                            Text(verbatim: Copy.newPlanStart.text)
                        }
                    }
                    if let error = model.error(for: "start_date") { Text(verbatim: error).bold() }
                    Toggle(isOn: $model.hasTarget) { Text(verbatim: Copy.newPlanHasTarget.text) }
                        .accessibilityIdentifier("new-plan-has-target")
                    if model.hasTarget {
                        DatePicker(selection: $model.target, displayedComponents: .date) {
                            Text(verbatim: Copy.newPlanTarget.text)
                        }
                    }
                    if let error = model.error(for: "target_date") { Text(verbatim: error).bold() }
                }
                Section {
                    Button(Copy.newPlanSave.text) {
                        Task { if let ref = await model.submit() { created(ref) } }
                    }
                    .buttonStyle(.borderedProminent)
                    .frame(minHeight: 44)
                    .disabled(model.saving)
                    .accessibilityIdentifier("new-plan-save")
                    if model.saving { LoadingView(label: Copy.newPlanSaving.text) }
                }
            }
            // The summary appears at the top after a refused create; bring it into view, as the web scrolls to it.
            .onChange(of: model.problems) { _, problems in
                if !problems.isEmpty { Task { @MainActor in withAnimation { scroller.scrollTo("summary", anchor: .top) } } }
            }
            }
            .navigationTitle(Copy.newPlanTitle.text)
            .inlineTitle()
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button(role: .cancel) { dismiss() } label: {
                        Label { Text(verbatim: Copy.newPlanCancel.text) } icon: { Image(systemName: "xmark") }
                    }
                    .accessibilityIdentifier("new-plan-cancel")
                }
            }
        }
    }
}
