# Til risikodirektør

*Risikodirektør: de risici, softwaren hjælper dig med at se, og risiciene ved at bruge den.*

## Risiko, den hjælper dig med at styre

- **Risici, problemer og ændringsanmodninger** registreret pr. plan; et register over problemer på tværs af læsbare planer; ændringsanmodninger med funktionsadskillelse.
- **Leveringssikkerhed og undtagelser:** angivet sikkerhed holdt adskilt fra den beregnede RAG, markeret, når de afviger to trin, med en historik, der kun kan tilføjes til, og et undtagelsesur på 48 timer, som ikke starter forfra, når en vurdering forværres, og som kun lukkes af en registreret beslutning.
- **Ærligt fravær:** manglende data vises som "ikke vurderet" eller null med en årsag, aldrig som grøn eller nul.
- **En frossen, versioneret månedlig pakke,** så det, direktionen fik at vide, senere kan vises præcis som udstedt.

## Risiko ved at bruge den (projektets eget register)

| Risiko | Tilstand |
|---|---|
| Ingen penetrationstest, revision fra tredjepart eller certificering | Åben: købers at bestille ([tjekliste til kvalitetssikring](../../../../governance/assurance-checklist.md)) |
| Opbevaring håndhæves kun for en del af dataene | Åben; sletningen skal planlægges |
| Værktøjet til registrerede søger ikke i fri tekst, vedhæftninger eller sikkerhedskopier | Åben |
| Revisionssporet kan omgås af en databaseadministrator; ingen hashkæde | Oplyst i [DPIA'en](../../../../governance/dpia.md) |
| Klassificeringsmærker skjuler og mærker, de krypterer ikke | Oplyst |
| Scanning af vedhæftninger kræver en scanner, som installatøren kører | Åben |
| Én vedligeholder, ingen serviceaftale | Åben |
| Oversættelserne er udkast | Åben |
| Kun ikke-klinisk; ingen klinisk sikkerhedsvurdering | Indtast ikke klinisk indhold |

## Kontroller, softwaren håndhæver

Login som standard, begrænsning af antal forespørgsler, strenge sikkerhedshoveder, en timeout ved inaktivitet i sessionen (30 minutter) og en fast grænse for sessionen, udgående destinationer begrænset til offentlig `https`, vedhæftninger med kontrolleret signatur, uforanderlige revisionsrækker og nødadgang (break-glass) med revision. Hver af dem er dækket af en test, der er nævnt i [vejen til produktion](../../path-to-production/index.md).

## Beslutninger, der er dine

Risikovilligheden for en pilot; undtagelsesvinduet (som standard 48 timer); hvilket klassificeringsniveau installationen er kvalitetssikret til; hvem der gennemgår adgangslisten, og hvor ofte.

## Læs videre

[Vejen til produktion](../../path-to-production/index.md) · [DPIA](../../../../governance/dpia.md) · [Overholdelse af regler](../../../../spec/12-compliance.md) · [SECURITY](../../../../SECURITY.md)
