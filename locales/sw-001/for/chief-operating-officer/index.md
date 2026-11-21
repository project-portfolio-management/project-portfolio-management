# Kwa Afisa Mkuu wa Uendeshaji

*Afisa Mkuu wa Uendeshaji: unachoendesha kila siku, programu inafanya nini kwa utekelezaji, na kinachohitajika kuiendesha.*

## Inafanya nini kwa uendeshaji

- **Kupanga na kudhibiti utekelezaji:** mipango, kazi, utegemezi, hesabu ya njia muhimu, hatua muhimu katika mipango, mbio na awamu, na vipimo vya burndown na mtiririko.
- **Uwezo:** mgawanyo, upatikanaji, likizo na vipindi visivyo vya kazi, majukumu ya rasilimali, mzigo wa kazi na matumizi yanayotokana. (Ramani ya joto ya uwezo kwa kundi la ujuzi na mwezi kwa pakiti ya wakurugenzi haijajengwa: `DP-10`.)
- **Muda na gharama:** karatasi za saa, muda uliorekodiwa, viwango vya gharama (vilivyozuiwa kwa fedha), bajeti na gharama inayotokana.
- **Mtiririko wa utawala:** rejista ya masuala, maombi ya mabadiliko yenye mgawanyo wa majukumu (mwombaji hawezi kuidhinisha), idhini, mapitio, kesi za wanaojiunga/wanaohama/wanaondoka.
- **Vighairi:** imani ya utekelezaji nyekundu au kaharabu-nyekundu hufungua kighairi kinachohitaji chanzo, uamuzi unaohitajika, mmiliki aliyetajwa na tarehe ya kuamua ndani ya saa 48 (dirisha linaweza kusanidiwa, saa 1 hadi 720). Ukiukaji huonekana kwenye pakiti ya wakurugenzi na katika `GET /api/exceptions`.

## Kinachohitajika kuiendesha

| Hitaji | Maelezo |
|---|---|
| Huduma | Huduma moja ya Rust, PostgreSQL 18, Redis ya hiari (akiba ya uzalishaji, vikao vya pamoja vya hiari), kiolesura cha SvelteKit |
| Mwongozo wa uendeshaji | [pakiti ya uendeshaji](../../../../operations/README.md): usambazaji, mwongozo wa uendeshaji, nakala rudufu na urejeshaji, malengo ya urejeshaji, uboreshaji na uhamishaji |
| Kazi zilizopangwa | Hakuna kinachojipangia chenyewe. Usafishaji wa uhifadhi, arifa za ukiukaji wa vighairi na kazi kama hizo lazima ziwekwe kwenye kipangaji na waendeshaji wako |
| Msaada | Mtunzaji mmoja, hakuna SLA; panga bajeti ya msaada wako wa mstari wa kwanza |

## Jua mipaka

- Mpango wa kazi elfu kadhaa ni polepole kupangiwa ratiba; jibu kamili hukataliwa zaidi ya kazi 5,000 isipoombwa (`?full=true` au `?only=critical`). Takwimu na tahadhari ziko katika [`BENCHMARKS.md`](../../../../BENCHMARKS.md); ni kiwango cha ukubwa kutoka mashine moja, si uwezo.
- Skrini zipo kwa sehemu kubwa ya eneo la kazi lakini **bado hazipo** kwa maombi ya mabadiliko, masuala, pakiti wala imani ya utekelezaji; hivyo ni API pekee leo.

## Maamuzi ambayo ni yako

Nani anamiliki kila kighairi, nani anatoa pakiti ya kila mwezi (mdundo na vikumbusho ni `DP-12`, havijajengwa), na ni mzunguko gani wa mapitio unaolishwa na kanuni ya kighairi.

## Soma ifuatayo

[Pakiti ya uendeshaji](../../../../operations/README.md) · [Mwongozo wa uendeshaji](../../../../operations/runbook.md) · [Malengo ya urejeshaji](../../../../operations/recovery-objectives.md)
