# Für die Technikleitung (CTO)

*Technikleitung (CTO): Architektur, Qualität, wie sie sich erweitern lässt und wo die Entwicklung dünn ist.*

## Architektur

Drei Teilprojekte, eine Einheit ([Architektur](../../../../spec/08-architecture.md)):

| Teil | Technik |
|---|---|
| Abgleichsbibliothek | Rust; deterministischer und probabilistischer, nachvollziehbarer paarweiser Abgleich; vom Dienst eingebettet |
| Dienst | Rust, axa auf axum, sqlx, PostgreSQL 18; REST unter `/api`, OpenAPI aus den Routen abgeleitet |
| Oberfläche | SvelteKit 3, Svelte-5-SPA mit einem Backend-for-Frontend; i18n-Kataloge |

Die Toolchain ist in [`rust-toolchain.toml`](../../../../rust-toolchain.toml) festgelegt. Die Persistenz steht in [Spezifikation 10](../../../../spec/10-persistence.md); die API in [Spezifikation 09](../../../../spec/09-api-surface.md).

## Technische Stärken, die man kennen sollte

- **Invarianten liegen in der Datenbank, wo sie zählen:** Das Prüfprotokoll weist Änderung und Löschung ab; herausgegebene Leitungspakete und Bewertungen der Lieferzuversicht sind per Trigger unveränderlich oder nur ergänzbar; CHECK-Constraints stützen die Regeln des Dienstes.
- **Wächtertests:** Zähltests (Census) lassen den Build scheitern, wenn eine neue aggregierende Route den Filter für private Pläne auslässt, eine Listenroute nicht seitenweise arbeitet, eine Spalte mit personenbezogenen Daten im Verzeichnis fehlt oder eine Route in OpenAPI fehlt. Sie werden behoben, indem das Neue eingeordnet wird, nicht indem der Test geschwächt wird.
- **Tests mit echten Tokens:** Die Autorisierung wird von Anfang bis Ende mit signierten PASETO-Tokens getestet (jeder Test in einem eigenen Prozess, weil die Authentifizierungseinstellungen einmal je Prozess gelesen werden).
- **Lieferkette:** `cargo deny`, eine Software-Stückliste, Build-Herkunft bei der Veröffentlichung, eine Sicherheitsrichtlinie.
- **CI** baut und testet bei jedem Push alles; siehe `.github/workflows/`.

## Wo sie dünn ist

- Eine Betreuerin oder ein Betreuer, also ein Bus-Faktor von eins ([`MAINTAINERS.md`](../../../../MAINTAINERS.md)).
- Die Leistung ist nur grob bekannt; der Antwortcache deckt nur die Route des kritischen Pfads ab; sehr große Pläne sind langsam.
- Sitzungen liegen im Arbeitsspeicher, sofern Redis nicht konfiguriert ist.
- Mehrere Abschnitte der Leitungspakete und alle Masken für Pakete sind nicht gebaut ([`tasks.md`](../../../../tasks.md)).
- KI-Werkzeuge werden zum Bau eingesetzt; das Produkt enthält kein Modell. Siehe die [KI-Erklärung](../../../../AI_STATEMENT.md).

## Entscheidungen, die bei Ihnen liegen

Betrieb und Redis-Topologie, ob geforkt oder dem Upstream gefolgt wird, wer den Code unabhängig prüft und wie der Dienst zu Ihrem Identitätsanbieter passt (PASETO-Schlüsselsatz oder OIDC über Keycloak oder Entra).

## Weiterlesen

[Architektur](../../../../spec/08-architecture.md) · [Teststrategie](../../../../spec/11-testing-strategy.md) · [Stand der Umsetzung](../../../../spec/14-implementation-status.md) · [Benchmarks](../../../../BENCHMARKS.md)
