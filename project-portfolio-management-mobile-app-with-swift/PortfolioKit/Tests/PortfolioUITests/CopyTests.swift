// MA-4: every word comes from the String Catalog, which has an English value for every key and no other language.

import Foundation
import PortfolioAPI
@testable import PortfolioUI
import Testing

@Suite struct CopyTests {
    static func catalog() throws -> [String: Any] {
        let url = URL(filePath: #filePath).deletingLastPathComponent().deletingLastPathComponent()
            .deletingLastPathComponent().appending(path: "Sources/PortfolioUI/Resources/Localizable.xcstrings")
        return try #require(JSONSerialization.jsonObject(with: Data(contentsOf: url)) as? [String: Any])
    }

    @Test func everyKeyHasAnEnglishValueAndNoOtherLanguage() throws {
        let catalog = try Self.catalog()
        #expect(catalog["sourceLanguage"] as? String == "en")
        let strings = try #require(catalog["strings"] as? [String: [String: Any]])
        for copy in Copy.allCases {
            let entry = try #require(strings[copy.rawValue], "no catalog entry for \(copy.rawValue)")
            let localizations = try #require(entry["localizations"] as? [String: Any])
            #expect(Array(localizations.keys) == ["en"], "\(copy.rawValue) has another language")
            let english = localizations["en"] as? [String: Any]
            let unit = english?["stringUnit"] as? [String: Any]
            let value = unit?["value"] as? String ?? ""
            #expect(!value.isEmpty, "\(copy.rawValue) has no English value")
        }
        #expect(Set(strings.keys) == Set(Copy.allCases.map(\.rawValue)), "the catalog has keys the code does not use")
    }

    @Test func theBundleResolvesEveryKey() {
        for copy in Copy.allCases {
            #expect(copy.text != copy.rawValue, "\(copy.rawValue) is not in the built catalog")
        }
    }

    @Test func placeholdersAreFilled() {
        #expect(Copy.taskPoints(5) == "Points: 5")
        #expect(Copy.plansNoResults("quay") == "No plans match “quay”.")
    }

    @Test func aCustomStatusIsShownByItsText() {
        #expect(PlanStatus.custom("Awaiting funding").label == "Awaiting funding")
        #expect(PlanStatus.onHold.label == "On hold")
        #expect(TaskStatus.inProgress.label == "In progress")
        #expect(TaskStatus.other("qa_signoff").label == "qa_signoff")
    }
}
