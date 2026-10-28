# MHRA medical device regulation: scoping assessment of Project Portfolio Management (PPM)

The [Medicines and Healthcare products Regulatory Agency](https://www.gov.uk/government/organisations/medicines-and-healthcare-products-regulatory-agency)
(MHRA) "regulates medicines, medical devices and blood components for transfusion in the UK". This page asks one
question of this repository: **is PPM a medical device, and if not, which MHRA duties fall away?** It was written from
the repository as it stood on **2026-10-28**, for the **service 0.3.0**, the **matcher library 0.2.0** and the **web
client 0.1.0**.

> ⚠️ **This is a self-assessment, written by the maintainer's AI coding assistant. Nobody independent has reviewed it,
> and it is not legal or regulatory advice.** The MHRA's own pages recommend professional advice. It is **not** a
> declaration of conformity and does not say PPM is exempt; it says what the *intended purpose* recorded in the
> repository would mean under the rules, so a deployer can challenge it.

## What was and was not read

| Source | Read | Not read |
| --- | --- | --- |
| [MHRA organisation page](https://www.gov.uk/government/organisations/medicines-and-healthcare-products-regulatory-agency) | Yes, on 2026-10-28 | |
| [Regulating medical devices in the UK](https://www.gov.uk/guidance/regulating-medical-devices-in-the-uk) | Yes (the summary below rests on it) | |
| [Register medical devices to place on the market](https://www.gov.uk/guidance/register-medical-devices-to-place-on-the-market) | Linked only | The page itself |
| [Medical devices: software applications (apps)](https://www.gov.uk/government/publications/medical-devices-software-applications-apps) | The landing page only (last updated 1 July 2023) | **The 43-page guidance PDF and its four appendices** (symptom checkers, clinical calculators, "drives or influences the use of a device", field safety notices). The intended-purpose test and the exclusions for software are in that PDF. |
| The Medical Devices Regulations 2002 (UK MDR 2002) | Named only | The text |
| [Yellow Card](https://www.gov.uk/yellowcard) and [drug and device alerts](https://www.gov.uk/drug-device-alerts) | Linked only | |

Until the software guidance PDF is read against the table below, **every "not applicable" answer is provisional**.

## The product's intended purpose

PPM is a portfolio and project management system for an organization's own work. It holds **no patient data, takes no
clinical input and gives no clinical output** ([DTAC assessment, "The product in one paragraph"](../national-health-service/digital-technology-assessment-criteria/2.0/index.md),
[spec/01-purpose-and-vision.md](../../../spec/01-purpose-and-vision.md)). It is open-source software that the buyer
deploys; the maintainer does not host it. Under the general rules, whether software is a medical device turns on the
manufacturer's stated intended purpose. Nothing in the repository states a medical purpose.

**The risk to watch** is a deployer who uses PPM for a clinical purpose (for example, to schedule or prioritise patient
treatment) and says so in their own materials. The intended purpose is then theirs, and so are the duties. Nothing in
the repository stops or detects that use.

## Duties under the general rules

| Duty (from "Regulating medical devices in the UK") | Applies to PPM? | Assessment | Evidence or gap |
| --- | --- | --- | --- |
| The product is a "medical device" (UK MDR 2002) | Only if its intended purpose is medical | **Not applicable, provisional** | Intended purpose is non-clinical; the software guidance was not read, so the exclusion is not confirmed against its test |
| Register the device with the MHRA before placing it on the Great Britain market | Only if it is a medical device | **Not applicable, provisional** | No registration exists, and none should be implied |
| UKCA or CE marking, with a UK approved body where third-party assessment is required | Only if it is a medical device | **Not applicable, provisional** | No marking is claimed anywhere in the repository |
| Appoint a UK Responsible Person (manufacturers outside the UK) | Only if it is a medical device | **Not applicable, provisional** | The maintainer's country is not published ([DTAC A9](../national-health-service/digital-technology-assessment-criteria/2.0/index.md)): **manufacturer to complete** |
| Classify the device and self-declare or use a UK approved body | Only if it is a medical device | **Not applicable, provisional** | Not attempted |
| Label with the marking and the UK Responsible Person | Only if it is a medical device | **Not applicable, provisional** | |
| Post-market surveillance and vigilance reports to the MHRA when certain incidents occur in the UK | Only if it is a medical device | **Not applicable, provisional** | The maintainer receives no telemetry and has no access to any deployment, so would not learn of an incident. If a deployer used PPM clinically, that would be a gap. |
| Safety action and field safety notices | Only if it is a medical device | **Not applicable, provisional** | [SECURITY.md](../../../SECURITY.md) covers security reports, not device incidents |

## What is not done

- No MHRA registration, UKCA or CE marking, declaration of conformity, classification, UK Responsible Person or
  vigilance procedure exists, and the above says why none is claimed.
- The software guidance PDF has not been read, so this page does **not** show that PPM falls outside the definition.
- No clinical-use warning is shown in the product or the README. Adding one is a decision for the maintainer.
- Nobody with regulatory expertise has reviewed this page.

Related: [DTAC v2.0 assessment](../national-health-service/digital-technology-assessment-criteria/2.0/index.md) (its clinical
safety answer, C1, rests on the same non-clinical intended purpose).
