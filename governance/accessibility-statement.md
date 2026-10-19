# Accessibility statement

> A template to publish after your own checks. Replace the `‹buyer›` markers.
> It is drawn from what the software's tests actually cover.

‹buyer organization› wants as many people as possible to be able to use this
service. The interface aims to meet the Web Content Accessibility Guidelines
(WCAG) **2.2, level AA**.

## What you can do

- zoom the page to 200% without losing content, and use it on a phone;
- use it with the keyboard alone;
- use it with a screen reader;
- turn on **simple mode** (the everyday destinations in front, the rest one step
  away) and a **dyslexia-friendly typeface** on the display preferences page;
  the interface also respects your system's **reduced-motion** setting, and you
  can choose a visual theme and a language (see the [language statement](language-statement.md)).

## How we checked

Each time the software changes, an automated audit with **axe-core** runs in a
real browser over **every page of the application** against the WCAG 2.0, 2.1
and **2.2** level A and AA rules that can be tested by machine (the rule sets
`wcag2a`, `wcag2aa`, `wcag21a`, `wcag21aa` and `wcag22aa`). The build fails if a
page has a serious or critical violation. A separate audit checks that the
phone layout has no horizontal scrolling.

Status is shown with a word **and** a shape, never colour alone; charts have a
table that carries the same figures; forms say what is wrong in words before
anything is sent.

## What the automated checks cannot show

Automated tools find roughly a third of accessibility problems. These are **not**
covered by the tests and need people:

- how the pages sound and read with a screen reader (NVDA, JAWS, VoiceOver);
- whether the reading and focus order makes sense on complex pages (the Gantt
  chart, the network diagram, the board);
- cognitive load, plain language, and the clarity of error messages;
- the content people enter, and the sentences the API generates (these are in
  English; see the language statement).

Before you publish this statement, have these checked by a person and record the
date, the tools and the findings here: **‹buyer: date, tester, tools, result›**.

## Known issues

The third-party **Lily Design System** widgets (`@lilydesignsystem`: data grid, Gantt chart, calendar,
kanban board) are used for some views. They are checked by the same audit, but their internal behaviour is
not under this project's control. Each of these views also has a table or list
version that carries the same information in plain markup (the "table twin"),
which is the accessible route for the Gantt, the board and the network
diagram. Use it if the interactive view does not work for you.

## Feedback and contact

If you find something you cannot use, tell us: **‹buyer: contact route and response time›**.
If we do not respond well, you can complain to **‹buyer: the enforcement body for your sector›**.

*Statement prepared: ‹date›. Last reviewed: ‹date›.*
