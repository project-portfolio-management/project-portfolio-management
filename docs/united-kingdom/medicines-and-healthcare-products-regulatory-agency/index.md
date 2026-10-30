# MHRA medical device regulation: scoping assessment of Project Portfolio Management (PPM)

The [Medicines and Healthcare products Regulatory Agency](https://www.gov.uk/government/organisations/medicines-and-healthcare-products-regulatory-agency)
(MHRA) "regulates medicines, medical devices and blood components for transfusion in the UK". This page asks one
question of this repository: **is PPM a medical device, and if not, which MHRA duties fall away?** It was written from
the repository as it stood on **2026-10-28**, for the **service 0.2.0**, the **matcher library 0.2.1** and the **web
client 0.2.0**.

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
| [Medical devices: software applications (apps)](https://www.gov.uk/government/publications/medical-devices-software-applications-apps) | The landing page, and **the guidance PDF "Medical device stand-alone software including apps (including IVDMDs)" v1.10g, pages 1 to 12 (introduction, device and medical-purpose flow charts, intended purpose, non-medical functions), 30 (post-market surveillance) and 37 (not a medical device)** | Pages 13 to 29 (the individual medical functions and in-vitro diagnostics), 31 to 36 (classification, essential requirements, labelling), 38 to 43, and the four appendices (symptom checkers, clinical calculators, "drives or influences the use of a device", field safety notices) |
| The Medical Devices Regulations 2002 (UK MDR 2002) | Named only | The text |
| [Yellow Card](https://www.gov.uk/yellowcard) and [drug and device alerts](https://www.gov.uk/drug-device-alerts) | Linked only | |

The answers below rest on the pages marked read. A reader who finds a function of PPM in the unread pages 13 to 29 should reopen them.

## What the guidance says, and how PPM falls against it

| Guidance (page) | What it says | PPM |
| --- | --- | --- |
| Computer program (7) | Software, "freeware / open-source software" included, can be a device; the rules cover every way of distributing it, because they apply to products "placed on the market" rather than sold. | Being free and open source gives **no exemption**. The test is the purpose. |
| Intended purpose (11) | A medical purpose "is determined by what the manufacturer states in the device's labelling, instructions for use and any promotional materials": adverts, app-store description, landing page, social media. A general disclaimer ("this product is not a medical device") is **not acceptable if medical claims are made or implied elsewhere**. The manufacturer's own view "is not solely determinative": an objective observer such as the MHRA can view a product as a device. | Nothing in the README, `spec/` or the docs claims a medical purpose. The repository does hold NHS DTAC material; it says in terms that PPM is non-clinical, but a reader could take its existence as a healthcare claim. |
| Medical purpose flow chart (10) | A device has one or more of: prevention, diagnosis, monitoring, treatment or alleviation, compensation, investigation or modification of anatomy, control of conception (and, for in-vitro data, further purposes). Software that **only** does things such as "stores or transmits medical data without change", an **administrative function** (booking an appointment, a record that "simply replaces a patient's paper file"), or "databases without internal language/macros/scripting" has no medical purpose. | PPM does none of the listed functions. It holds portfolio and project records. |
| Non-medical functions (12) | "MHRA do not generally regulate data, databases or analytical services, but if you are analysing or processing data for a medical purpose, then the software that you are using may be covered." Decision-support software that applies a calculation or algorithm to clinical data is usually a device. | PPM computes schedules, costs and risk from project data, not clinical data. If a deployer fed it clinical data for a clinical decision, this is the passage that would bite. |
| Multipurpose product (10) | A multipurpose product "which is used occasionally in a medical environment" is normally not a device, unless it has a specific clinical intended purpose. | PPM is a general-purpose product; use in a hospital's project office does not change that. |
| Not a medical device (37) | Other law may apply, such as the General Product Safety Regulations, and manufacturers "must also consider" the UK GDPR and the Privacy and Electronic Communications Regulations. | Data protection is covered in the [DTAC assessment](../national-health-service/digital-technology-assessment-criteria/2.0/index.md) (C2) and [governance/](../../../governance/). **The General Product Safety Regulations were not assessed.** |

## The product's intended purpose

PPM is a portfolio and project management system for an organization's own work. It holds **no patient data, takes no
clinical input and gives no clinical output** ([DTAC assessment, "The product in one paragraph"](../national-health-service/digital-technology-assessment-criteria/2.0/index.md),
[spec/01-purpose-and-vision.md](../../../spec/01-purpose-and-vision.md)). It is open-source software that the buyer
deploys; the maintainer does not host it. Against the guidance's test (page 10), none of the listed medical
functions applies. **On the pages read, PPM has no medical purpose and is not a medical device.**

**The risk to watch** is a deployer who uses PPM for a clinical purpose (for example, to schedule or prioritise patient
treatment) and says so in their own materials. The guidance makes the manufacturer's stated purpose decisive, so the
intended purpose, and the duties, would then be the deployer's. Nothing in the repository stops or detects that use.

## Duties under the general rules

| Duty (from "Regulating medical devices in the UK") | Applies to PPM? | Assessment | Evidence or gap |
| --- | --- | --- | --- |
| The product is a "medical device" (UK MDR 2002) | Only if its intended purpose is medical | **Not applicable** | Intended purpose is non-clinical; tested against the guidance's medical-purpose flow chart (page 10), see above |
| Register the device with the MHRA before placing it on the Great Britain market | Only if it is a medical device | **Not applicable** | No registration exists, and none should be implied |
| UKCA or CE marking, with a UK approved body where third-party assessment is required | Only if it is a medical device | **Not applicable** | No marking is claimed anywhere in the repository |
| Appoint a UK Responsible Person (manufacturers outside the UK) | Only if it is a medical device | **Not applicable** | The maintainer's country is not published ([DTAC A9](../national-health-service/digital-technology-assessment-criteria/2.0/index.md)): **manufacturer to complete** |
| Classify the device and self-declare or use a UK approved body | Only if it is a medical device | **Not applicable** | Not attempted |
| Label with the marking and the UK Responsible Person | Only if it is a medical device | **Not applicable** | |
| Post-market surveillance and vigilance reports to the MHRA when certain incidents occur in the UK | Only if it is a medical device | **Not applicable** | The maintainer receives no telemetry and has no access to any deployment, so would not learn of an incident. The guidance (page 30) requires post-market surveillance and reporting to the MHRA only of a manufacturer of a device. If a deployer used PPM clinically, that would be a gap. |
| Safety action and field safety notices | Only if it is a medical device | **Not applicable** | [SECURITY.md](../../../SECURITY.md) covers security reports, not device incidents |

## What is not done

- No MHRA registration, UKCA or CE marking, declaration of conformity, classification, UK Responsible Person or
  vigilance procedure exists, and the above says why none is claimed.
- Only pages 1 to 12, 30 and 37 of the software guidance were read (flowchart images, by the assistant). Pages 13 to 29,
  31 to 36 and 38 to 43 and the appendices were not, and the UK MDR 2002 text and the registration page were not.
- The General Product Safety Regulations, which the guidance (page 37) points to for a non-device, were not assessed.
- The guidance is dated: the PDF is v1.10g and the landing page was last updated 1 July 2023. The MHRA said new
  legislation strengthening vigilance came into force on 16 June 2025. **Whether the software guidance has changed since was not checked.**
- No clinical-use warning is shown in the product or the README. Adding one is a decision for the maintainer.
- Nobody with regulatory expertise has reviewed this page.

Related: [DTAC v2.0 assessment](../national-health-service/digital-technology-assessment-criteria/2.0/index.md) (its clinical
safety answer, C1, rests on the same non-clinical intended purpose).
