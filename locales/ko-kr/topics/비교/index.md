# 비교

*이 소프트웨어가 다른 제품과 어떻게 관련되는지 요약합니다.*

아래 제품들은 목적이 서로 다르며 점수를 매기지 않았습니다. 이 설명은 각 프로젝트의 공개 자료를 바탕으로 합니다. **다른 제품은 설치하거나 실행해 보지 않았으며**, 그 뒤로 달라졌을 수 있습니다. 결정을 내리기 전에 최신 제품 문서를 확인해야 합니다.

| 프로젝트 | 무엇인가 | 이 프로젝트와 다른 점 |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | 프로젝트 관리자가 아닌 사람들을 위한 프로젝트 관리 | 이 프로젝트는 포트폴리오 사무국이 쓰는 일정, 비용, 거버넌스 기능도 포함합니다 |
| [Redmine](https://www.redmine.org/) | 조직이 이슈 유형, 상태, 역할을 정의하는 설정 가능한 범용 추적기 | 이슈 유형, 상태, 워크플로는 조직이 정의합니다. 역할에는 이슈 권한이 있으며, 사용자의 로그인이 허용하는 범위 안에서 계획별로 부여됩니다. [Redmine 비교](../../../en-001/compare/redmine/index.md)를 참고하세요 |
| Huly Platform (포크인 Platform Collective를 통해) | 여러 서비스로 이루어진 비즈니스 애플리케이션 프레임워크 | Huly는 다중 서비스 플랫폼입니다. 이 프로젝트는 PostgreSQL 위의 서비스 하나입니다 |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | 관리자 한 명과 프로젝트 하나를 위한 Microsoft Project의 데스크톱 대안 | 이 프로젝트는 접근 통제를 갖춘 호스팅형 다중 사용자 서비스입니다 |

## 제품별 비교

개별 비교 페이지(영어): [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## 상호 운용성

- **Jira와 Asana:** CSV 가져오기. 미리 보기를 제공하고 제외한 모든 항목의 이유를 밝힙니다.
- **Microsoft Project:** MSPDI XML 가져오기와 내보내기. 작업, 개요, 날짜, 기간, 제약 조건, 의존 관계, 달력을 다룹니다. 담당자, 자원, 기준선은 제외되며 파일에 그 사실이 기록됩니다.

## 설계 차이

- **명시된 이유.** 빠진 수치는 이유와 함께 표시됩니다. 예를 들어 "예측 없음, 왜냐하면…"처럼 나타납니다.
- **설명 가능한 중복 탐지.** 일치 탐지는 모든 계획 유형에 걸쳐 작동하고 근거를 보여 줍니다.
- **보안 기본값.** 로그인이 기본으로 켜져 있고 보안 자료가 저장소에 들어 있습니다.
- **명시적인 기간.** 모든 기간은 달력일 또는 영업일로 명시됩니다.
- **기록된 한계.** 백로그가 각 항목의 한계를 기록합니다.

## 알려진 한계

- 모바일 앱도 데스크톱 애플리케이션도 없습니다.
- 실시간 공동 편집이 없습니다.
- 유지 관리자는 한 명이며, 운영 이력이 제한적이고 제3자 감사가 없습니다.
- 공개된 부하 테스트는 한 대의 기계에서 실행되었습니다. 대략적인 규모를 보여 줄 뿐 용량 계획이 아닙니다.
- 번역은 검토를 기다리는 초안입니다.
- 변경 요청을 작성하거나 결정하는 화면이 없습니다.
- 플러그인이나 마켓플레이스 생태계가 없습니다.

자세한 내용은 [비교 노트](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md)에 있습니다. 정정은 출처를 붙여 이슈로 제출할 수 있습니다.
