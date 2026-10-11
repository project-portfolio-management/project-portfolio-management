# Llywodraethu, Risg, Cydymffurfiaeth

*I'r rhai sy'n rhedeg llywodraethu, risg a chydymffurfiaeth: yr hyn y mae'r meddalwedd yn eich helpu i'w brofi, yr hyn y mae'n ei orfodi, a'r hyn y mae'n rhaid i chi ei ddarparu. Nid cyngor cyfreithiol mo hwn.*

## Llywodraethu

- **Cofnodir penderfyniadau.** Mae cais newid angen cymeradwywr a enwir (nid y sawl sy'n gofyn), rheswm o 10 nod o leiaf, ac fe'i harchwilir yn ôl gweithredwr; mae'r gronfa ddata yn gwrthod cymeradwyaeth heb benderfynwr a rheswm.
- **Pecyn cyfarwyddwyr misol wedi'i rewi.** Drafftir unwaith, cyhoeddir gan unigolyn, anghyfnewidiol wedi hynny; ailgyhoeddi gyda nodyn yw cywiriad. Mynediad darllen yn ôl cynulleidfa.
- **Hyder cyflawni ac eithriadau.** Mae hyder a nodir yn ychwanegu-yn-unig ac fe'i dangosir wrth ymyl y statws a gyfrifir, byth wedi'i gymysgu. Mae sgôr coch neu ambr-coch yn agor eithriad 48 awr (y gellir ei ffurfweddu) na all ond penderfyniad cofnodedig ei gau.
- **Llywodraethu prosiect y meddalwedd ei hun:** [GOVERNANCE.md](../../../../GOVERNANCE.md), [rheoli newid](../../../../spec/18-change-control.md), [datganiad AI](../../../../AI_STATEMENT.md).

## Risg

- Risgiau, materion a cheisiadau newid fesul cynllun; cofrestr materion ar draws cynlluniau darllenadwy.
- Dangosir data coll fel "heb ei asesu", byth yn wyrdd.
- Mae risgiau agored y prosiect ei hun (dim prawf treiddio, cadw rhannol, un cynhaliwr, cyfieithiadau drafft) yn [parodrwydd cynhyrchu](../../../../governance/production-readiness.md). Casgliad: yn barod ar gyfer peilot rheoledig ar ddata anghlinigol, nad yw'n gyfrinachol, nid ar gyfer defnydd cynhyrchu heb oruchwyliaeth.

## Cydymffurfiaeth

| Maes | Lle | Cyflwr |
|---|---|---|
| GDPR y DU / GDPR yr UE | [DPIA](../../../../governance/dpia.md), [cofnod prosesu](../../../../governance/record-of-processing.md), [rhestr eiddo data personol](../../../../governance/personal-data-inventory.csv), [amserlen gadw](../../../../governance/retention-schedule.md), [llif data](../../../../governance/data-flow.md) | Wedi'u llenwi ymlaen llaw; mae marcwyr `‹buyer›` yn eiddo i chi; cadw wedi'i orfodi'n rhannol |
| DTAC y GIG | [Rhestr wirio DTAC](../../../../governance/dtac-checklist.md) | Atebwyd adran wrth adran; rhesi'r prynwr a'r cyflenwr yn agored |
| Hygyrchedd | [Datganiad](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA yn awtomatig; dim prawf â llaw |
| Dyletswyddau iaith | [Datganiad iaith](../../../../governance/language-statement.md) | Drafftiau yw'r cyfieithiadau |
| ISO/IEC 27001 | [Manyleb cydymffurfiaeth](../../../../spec/12-compliance.md) | Rheolaethau ochr y defnyddiwr; heb ei ardystio |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [Manyleb cydymffurfiaeth](../../../../spec/12-compliance.md) §12.4 | Ddim yn berthnasol (dim data cleifion); nodir, nid ei ffugio |

## Tystiolaeth a sicrwydd

Mae'r [rhestr wirio sicrwydd](../../../../governance/assurance-checklist.md) yn rhestru 14 eitem y bydd asesydd yn gofyn amdanynt, gyda pherchennog a chyflwr. Mae'r rhan fwyaf yn agored ac yn perthyn i'r defnyddiwr: prawf treiddio, archwiliad hygyrchedd, ymarfer adfer ar ôl trychineb, sicrwydd gwesteio, ffurfweddu'r darparwr hunaniaeth, ailardystio mynediad, proses digwyddiadau, rheoli cofnodion. Mae'r gronfa ddata yn gwrthod newid a dileu'r llwybr archwilio, ond gall gweinyddwr cronfa ddata ei drechu; dim cadwyn hash.

## Penderfyniadau sy'n eiddo i chi

Y sail gyfreithlon a'r cyfnodau cadw; y ffenestr eithriad; lefel marcio'r defnydd; pwy sy'n adolygu mynediad a pha mor aml; pa un a oes angen gweithdrefn daliad cyfreithiol (nid oes un wedi'i hadeiladu).

## Darllenwch nesaf

[Mynegai'r pecyn llywodraethu](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Manyleb cydymffurfiaeth](../../../../spec/12-compliance.md)
