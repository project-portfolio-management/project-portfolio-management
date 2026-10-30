# ISO 14971 scoping assessment of Project Portfolio Management (PPM)

ISO 14971 is the international standard for **risk management of medical devices** (current edition: ISO 14971:2019;
in Europe, EN ISO 14971:2019/A11:2021, whose technical content is the same). This page asks what it requires of this
repository. It was written from the repository as it stood on **2026-10-28**, for the **service 0.2.0**, the **matcher
library 0.2.1** and the **web client 0.2.0**.

> ⚠️ **This is a self-assessment, written by the maintainer's AI coding assistant. Nobody independent has reviewed it,
> and it is not regulatory advice.** **The standard itself was not read:** it is sold by ISO and was not available
> here. The clause headings below are from the assistant's knowledge of the 2019 edition and **have not been checked
> against the text**; the edition, scope and amendment facts come from public search results and the
> [MHRA scoping page](../../united-kingdom/medicines-and-healthcare-products-regulatory-agency/index.md) this page
> builds on. A reader with the standard should check every clause number before relying on this page. A search result
> also mentioned that a revision of the standard is under way; **whether it has been published was not checked.**

## Does it apply?

ISO 14971 is applied by the **manufacturer of a medical device** (software as a medical device and in-vitro diagnostic
devices included), across the device's life cycle. Whether PPM is a device was assessed in the
[MHRA scoping assessment](../../united-kingdom/medicines-and-healthcare-products-regulatory-agency/index.md): on the
pages of the MHRA software guidance that were read, **it is not**, because it has no medical purpose, so the
standard's process is **not required of it**. [spec/12-compliance.md](../../../spec/12-compliance.md) reaches the same
view ("not a device, and no clinical hazard") and says what is done instead: supply-chain and configuration evidence.

That view holds only while **no one states a medical purpose for PPM**. If a deployer does (for example, to prioritise
patients), the deployer becomes the manufacturer of that use, and the standard applies to them. PPM would then lack
everything in the table below.

## The standard's process against the repository

"Risk" in PPM means **project risk** (a risks register with scoring, schedule and cost simulation). ISO 14971 means
**the combination of probability and severity of harm** to patients, users or others. They are different things, and
the project risks register is **not** evidence for any row below.

| Clause (2019 edition, from memory) | What it asks | In this repository | Assessment |
| --- | --- | --- | --- |
| 4 General requirements: risk management process, management responsibilities, competence, risk management plan and file | A defined process, top management commitment, competent people, a plan and a file per device | None. One maintainer; no risk management plan or file | **Not applicable; not built** |
| 5 Risk analysis: intended use and reasonably foreseeable misuse, safety characteristics, hazards and hazardous situations, risk estimation | Find what could harm, and how likely and severe it is | Intended use is recorded and is non-clinical ([DTAC assessment](../../united-kingdom/national-health-service/digital-technology-assessment-criteria/2.0/index.md)). **Foreseeable misuse (clinical use) is named but not analysed.** No hazard list | **Partly: intended use only** |
| 6 Risk evaluation | Decide whether each risk is acceptable against set criteria | No acceptability criteria for harm | **Not applicable; not built** |
| 7 Risk control: options, implementation, residual risk, benefit-risk, new risks from controls, completeness | Reduce risk by design, protection or information, and verify it | Not as harm control. Data-integrity and security controls exist ([governance/production-readiness.md](../../../governance/production-readiness.md)) but are not tied to harm | **Not applicable; not built** |
| 8 Evaluation of overall residual risk | Judge the whole device | None | **Not applicable; not built** |
| 9 Risk management review | Review before release that the plan was carried out | None for harm. Releases run CI gates ([AGENTS.md](../../../AGENTS.md)) | **Not applicable; not built** |
| 10 Production and post-production activities | Collect and review field information, and act on it | No telemetry and no access to any deployment, so the maintainer would not learn of an incident ([MHRA page](../../united-kingdom/medicines-and-healthcare-products-regulatory-agency/index.md)); [SECURITY.md](../../../SECURITY.md) covers security reports only | **Not applicable; a gap if used clinically** |

## What a deployer who uses PPM clinically would need

A [starting hazard analysis](hazard-analysis.md) names the hazards a clinical use would add to and the controls PPM has. The following are not done and not planned; they are listed so the gap is plain:

- a stated intended purpose, a hazard analysis and acceptability criteria for the clinical use;
- the software life-cycle process the standard is normally paired with (IEC 62304) and a register of third-party
  software of unknown provenance. The service spec records that **no SOUP register exists**
  ([service spec](../../../project-portfolio-management-service-with-rust/spec/index.md)); the project does check a
  CycloneDX software bill of materials against its lock files (`scripts/check-sbom.py`);
- the guidance on applying the standard (ISO/TR 24971), and the NHS clinical-safety standards (DCB0129 for the manufacturer, DCB0160 for the deploying
  organisation); the DTAC assessment records that **no DCB0129 documentation exists**, and says nothing of DCB0160.

## What is not done

- The standard was not read, and no clause number here is checked.
- No risk management plan, file, hazard list or acceptability criteria exist, and none is claimed.
- No clinical-use warning was added to the product or README. That is the maintainer's decision.
- Nobody with regulatory or clinical-safety expertise has reviewed this page.
