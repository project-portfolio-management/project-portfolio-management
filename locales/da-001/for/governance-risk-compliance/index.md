# Governance, risiko og overholdelse

*Til dem, der driver governance, risiko og overholdelse: hvad softwaren hjælper dig med at dokumentere, hvad den håndhæver, og hvad du selv må levere. Ikke juridisk rådgivning.*

## Governance

- **Beslutninger registreres.** Ændringsanmodninger kræver en navngiven godkender, der ikke er anmoderen, en angivet begrundelse på mindst 10 tegn, og de revideres efter aktør; databasen afviser en godkendelse uden en beslutningstager og en begrundelse.
- **En frossen månedlig direktionspakke.** Udarbejdet én gang, udstedt af en person, uforanderlig, når den er udstedt; rettelser er en genudstedelse med en note. Læseadgang gives efter målgruppe.
- **Leveringssikkerhed og undtagelser.** Angivet sikkerhed kan kun tilføjes til og vises ved siden af beregnet status, aldrig blandet med den. En rød eller ravgul-rød vurdering åbner en undtagelse med et ur på 48 timer (kan konfigureres), som kun en registreret beslutning lukker.
- **Projektgovernance for selve softwaren:** [GOVERNANCE.md](../../../../GOVERNANCE.md), [ændringskontrol](../../../../spec/18-change-control.md), [AI-erklæring](../../../../AI_STATEMENT.md).

## Risiko

- Risici, problemer og ændringsanmodninger pr. plan; et register over problemer på tværs af læsbare planer.
- Manglende data vises som "ikke vurderet", aldrig som grøn.
- Projektets egne åbne risici (ingen penetrationstest, delvis opbevaring, én vedligeholder, udkast til oversættelser) står på siden [vejen til produktion](../../path-to-production/index.md), punkt for punkt med et estimat for hvert.

## Overholdelse

| Område | Hvor | Tilstand |
|---|---|---|
| Databeskyttelse (den gældende lov; [§12.2a](../../../../spec/12-compliance.md) kortlægger GDPR, UK GDPR, LGPD, PIPL, DPDP, PIPEDA, CCPA og andre) | [DPIA](../../../../governance/dpia.md), [fortegnelse over behandlingsaktiviteter](../../../../governance/record-of-processing.md), [fortegnelse over personoplysninger](../../../../governance/personal-data-inventory.csv), [opbevaringsplan](../../../../governance/retention-schedule.md), [datastrøm](../../../../governance/data-flow.md) | Forhåndsudfyldt; `‹buyer›`-markeringer er dine; opbevaring kun delvist håndhævet |
| Britiske NHS DTAC (én national ordning, et gennemarbejdet eksempel; andre ordninger starter fra [tjeklisten til kvalitetssikring](../../../../governance/assurance-checklist.md)) | [DTAC-tjekliste](../../../../governance/dtac-checklist.md) | Besvaret afsnit for afsnit; rækker for køber og leverandør er åbne |
| Tilgængelighed | [erklæring](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA automatisk; ingen manuel test |
| Sprogkrav | [sprogerklæring](../../../../governance/language-statement.md) | Oversættelserne er udkast |
| ISO/IEC 27001 | [specifikation om overholdelse](../../../../spec/12-compliance.md) | Kontroller på installationssiden; ikke certificeret |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [specifikation om overholdelse](../../../../spec/12-compliance.md) §12.4 | Ikke berørt (ingen patientdata); oplyst, ikke opdigtet |

## Dokumentation og kvalitetssikring

[Tjeklisten til kvalitetssikring](../../../../governance/assurance-checklist.md) opregner 14 punkter, en vurderingsmand vil bede om, med ejer og tilstand. De fleste er åbne og tilhører den, der installerer: penetrationstest, tilgængelighedsrevision, katastrofeøvelse, sikring af hosting, konfiguration af identitetsudbyder, recertificering af adgang, hændelsesproces, arkivstyring. Revisionssporet afviser ændring og sletning i databasen, men en databaseadministrator kan omgå det; der er ingen hashkæde.

## Beslutninger, der er dine

Behandlingsgrundlag og opbevaringsperioder; undtagelsesvindue; klassificeringsniveau for installationen; hvem der gennemgår adgang, og hvor ofte; om der er brug for en procedure for juridisk tilbageholdelse (ingen er bygget).

## Læs videre

[Oversigt over governance-pakken](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Specifikation om overholdelse](../../../../spec/12-compliance.md)
