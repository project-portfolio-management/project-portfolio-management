# 튜토리얼

*로컬 컴퓨터에서 시연을 실행하고 샘플 포트폴리오를 살펴보는 단계.*

시연은 소프트웨어의 완전한 사본으로, 가상의 조직인 노스게이트 교통 당국의 데이터가 들어 있습니다. 모든 사람, 계획, 수치, 날짜는 가상이며 데이터는 자유롭게 바꿀 수 있습니다.

**보안 참고.** 시연은 공개된 테스트 키를 신뢰합니다. 공유 네트워크에서 실행하거나 실제 데이터와 함께 사용해서는 안 됩니다.

## 요구 사항

- [Git](https://git-scm.com/)
- Compose가 포함된 [Podman](https://podman.io/)

## 1단계: 코드 내려받기

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## 2단계: 시연 시작

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

이 명령은 소프트웨어를 빌드하고, 데이터베이스를 시작하고, 가상의 포트폴리오를 불러오고, 인터페이스를 시작합니다. 처음 빌드는 이후보다 오래 걸립니다.

## 3단계: 로그인

<http://localhost:3000/ko-001/signin>을 열고, 주소를 `example@example.com`으로 그대로 둔 채 **Sign in to the demonstration**을 누르세요. 이메일은 전송되지 않습니다.

첫 화면은 **시연 투어**로, 모든 기능을 그룹별로 설명과 링크와 함께 나열합니다. 모든 페이지의 노란색 배너는 데이터가 가상임을 알립니다.

## 4단계: 포트폴리오 살펴보기

투어에서 **Plans**를 열어 포트폴리오와 그 프로그램, 프로젝트를 봅니다. 그런 다음 **Portfolio dashboard**를 엽니다.

## 5단계: 프로젝트 하나 살펴보기

투어의 수행 그룹에서 **North Line interlocking** 프로젝트를 엽니다. 그 **Gantt** 차트, **Critical path**, **Earned value**를 살펴보세요.

## 6단계: 개인 업무 살펴보기

**My work**와 **My kanban board**를 엽니다. 세션은 Example Person으로 로그인되어 있으며, 이 사람에게는 진행 중인 작업이 있습니다.

## 7단계: 역량, 시작 통제, 보고 살펴보기

**Capacity heatmap**을 열어 초과 배정된 자원을 봅니다. **Start queue**를 열면 진행 중 업무 한도가 프로그램 하나를 대기시킨 것을 볼 수 있습니다. 한 번 발행되면 고정되는 월간 요약인 **Director packs**를 엽니다.

## 8단계: 시연 중지

이 명령은 모든 서비스를 중지하고 시연 데이터를 삭제합니다.

```sh
podman compose --profile demo down -v
```

## 다음 단계

- [설치 안내서](../../../../INSTALL.md): 로그인과 ID 공급자를 다룹니다
- 고위 역할을 위한 [역할별 안내서](../../for/)
- [시연 안내서](../../../../operations/demo.md)
- [기능 목록](../기능/index.md)

문제는 [이슈 추적기](https://github.com/project-portfolio-management/project-portfolio-management/issues)로 신고할 수 있습니다.
