// MA-1.4: one plan's fields, its child plans and its tasks; MA-1.5: a task moved to another status from a sheet. Status is
// always in words (a Lily `Tag`, the record's status line), never colour alone (MA-4).

import LilyUI
import PortfolioAPI
import SwiftUI

struct PlanScreen: View {
    let app: AppModel
    @State private var model: PlanViewModel
    @State private var moving: PlanTask?

    init(ref: PlanRef, app: AppModel) {
        self.app = app
        _model = State(initialValue: PlanViewModel(ref: ref, app: app))
    }

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 24) {
                switch model.state {
                case .loading:
                    LoadingView()
                case .failed(let presentation):
                    FailureView(presentation: presentation) { await model.load() }
                case .loaded(let content):
                    details(content.plan)
                    children(content.children)
                    tasks(content.tasks, truncated: content.truncated)
                }
            }
            .padding()
            .frame(maxWidth: .infinity, alignment: .leading)
        }
        .navigationTitle(model.ref.name)
        .inlineTitle()
        .task { await model.load() }
        .refreshable { await model.load() }
        .sheet(item: $moving) { task in
            MoveTaskSheet(task: task, model: model) { moving = nil }
        }
    }

    private func details(_ plan: Plan) -> some View {
        Card(heading: Copy.planDetails.text, headingLevel: .h2) {
            SummaryList(label: Copy.planDetails.text) {
                SummaryListItem(term: Copy.planStatus.text) {
                    if let status = plan.status {
                        Tag(status.label, label: Copy.planStatusTagLabel.text)
                    } else {
                        Text(verbatim: Copy.planNotSet.text)
                    }
                }
                item(Copy.planKind.text, plan.kind?.label)
                item(Copy.planCode.text, plan.code)
                item(Copy.planOwner.text, plan.ownerOrgName)
                item(Copy.planStart.text, plan.startDate?.formatted())
                item(Copy.planTarget.text, plan.targetDate?.formatted())
            }
        }
    }

    private func item(_ term: String, _ value: String?) -> some View {
        SummaryListItem(term: term) { Text(verbatim: value ?? Copy.planNotSet.text) }
    }

    private func children(_ children: [PlanRef]) -> some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(verbatim: Copy.planChildren.text)
                .font(.title3.bold())
                .accessibilityAddTraits(.isHeader)
            if children.isEmpty {
                Text(verbatim: Copy.planNoChildren.text)
            }
            ForEach(children) { child in
                NavigationLink(value: Route.plan(child)) {
                    HStack {
                        Text(verbatim: child.name).multilineTextAlignment(.leading)
                        Spacer(minLength: 8)
                        Image(systemName: "chevron.forward").accessibilityHidden(true)
                    }
                    .frame(maxWidth: .infinity, minHeight: 44, alignment: .leading)
                    .contentShape(Rectangle())
                }
                .accessibilityIdentifier("child-row-\(child.pid)")
            }
        }
    }

    private func tasks(_ tasks: [PlanTask], truncated: Bool) -> some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(verbatim: Copy.planTasks.text)
                .font(.title3.bold())
                .accessibilityAddTraits(.isHeader)
            RecordList(
                status: tasks.isEmpty ? .noneKnown : .known,
                statusLabel: tasks.isEmpty ? Copy.planNoTasks.text : Copy.planTasks.text,
                label: Copy.planTasks.text,
                coverageLabel: truncated ? Copy.planTasksTruncated(tasks.count) : nil
            ) {
                ForEach(tasks) { task in
                    Button { moving = task } label: {
                        RecordListItem(
                            status: task.status.wireName, statusLabel: Copy.taskStatusLabel(task.status.label),
                            updatedLabel: secondLine(task)
                        ) {
                            Text(verbatim: task.title).font(.body.weight(.semibold)).multilineTextAlignment(.leading)
                        }
                        .frame(maxWidth: .infinity, minHeight: 44, alignment: .leading)
                        .contentShape(Rectangle())
                    }
                    .buttonStyle(.plain)
                    .accessibilityHint(Text(verbatim: Copy.taskMoveHint.text))
                    .accessibilityIdentifier("task-row-\(task.pid)")
                    Divider()
                }
            }
        }
    }

    private func secondLine(_ task: PlanTask) -> String? {
        let parts = [
            task.dueDate.map { Copy.taskDue($0.formatted()) },
            task.points.map { Copy.taskPoints($0) },
            task.blockedDays.map { Copy.taskBlockedDays($0) },
        ].compactMap { $0 }
        return parts.isEmpty ? nil : parts.joined(separator: " · ")
    }
}

/// MA-1.5: the statuses to move a task to. The task's own row says **Current status** in words.
struct MoveTaskSheet: View {
    let task: PlanTask
    let model: PlanViewModel
    let close: () -> Void

    var body: some View {
        NavigationStack {
            List {
                if let error = model.moveError {
                    Section { LilyAlert(kind: .error, heading: Copy.moveFailed.text, message: error.message) }
                }
                Section {
                    Text(verbatim: task.title).font(.headline)
                    if model.moving == task.pid {
                        LoadingView(label: Copy.moveMoving.text)
                    }
                    ForEach(TaskStatus.board) { status in
                        Button {
                            Task { if await model.move(task, to: status) { close() } }
                        } label: {
                            HStack {
                                Text(verbatim: status.label)
                                Spacer(minLength: 8)
                                if status == task.status {
                                    Text(verbatim: Copy.moveCurrent.text).font(.footnote)
                                    Image(systemName: "checkmark").accessibilityHidden(true)
                                }
                            }
                            .frame(maxWidth: .infinity, minHeight: 44, alignment: .leading)
                            .contentShape(Rectangle())
                        }
                        .disabled(model.moving != nil)
                        .accessibilityIdentifier("move-\(status.wireName)")
                    }
                }
            }
            .navigationTitle(Copy.moveTitle.text)
            .inlineTitle()
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button(role: .cancel, action: close) {
                        Label { Text(verbatim: Copy.moveCancel.text) } icon: { Image(systemName: "xmark") }
                    }
                    .accessibilityIdentifier("move-cancel")
                }
            }
        }
        .onAppear { model.moveError = nil }
    }
}
