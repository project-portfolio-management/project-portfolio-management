# 比較

*概述本軟體與其他產品的關係。*

下列產品用途各不相同，不作評分。這些說明基於各專案的公開資料。**未安裝或執行過任何其他產品**，各產品此後可能已有變化。作出決定前應查閱產品的最新文件。

| 專案 | 它是什麼 | 本專案的不同之處 |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | 面向非專案經理的專案管理工具 | 本專案還包括組合管理辦公室使用的進度、成本和治理功能 |
| [Redmine](https://www.redmine.org/) | 可配置的通用工具，由組織定義問題型別、狀態和角色 | 問題型別、狀態和工作流由組織定義。角色帶有問題許可權，按計劃分配，且不超出登入身份允許的範圍。參見[與 Redmine 的比較](../../../en-001/compare/redmine/index.md)（英文） |
| Huly Platform（透過其分支 Platform Collective） | 基於多個服務的業務應用框架 | Huly 是多服務平臺。本專案是一個基於 PostgreSQL 的服務 |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Microsoft Project 的桌面替代品，面向一位經理和一個專案 | 本專案是一項託管的多使用者服務，帶有訪問控制 |

## 產品比較

各產品比較頁面（英文）：[Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md)。

## 互操作性

- **Jira 和 Asana：** CSV 匯入，提供預覽，並說明所有未匯入內容的原因。
- **Microsoft Project：** 匯入和匯出 MSPDI XML，涵蓋任務、大綱、日期、工期、約束、依賴關係和日曆。任務負責人、資源和基線不包括在內，檔案中會記錄這一點。

## 設計差異

- **說明原因。** 缺失的數字附帶原因，例如「無預測，因為……」。
- **可解釋的重複檢測。** 匹配適用於所有型別的計劃，並展示證據。
- **安全預設設定。** 預設啟用登入，倉庫包含安全資料。
- **明確的工期。** 每個工期都註明日曆日或工作日。
- **記錄侷限。** 待辦清單記錄了每一項的侷限。

## 已知侷限

- 沒有移動應用或桌面應用。
- 沒有實時協同編輯。
- 只有一位維護者，生產歷史有限，沒有第三方審計。
- 已釋出的負載測試在一臺機器上執行，只表示數量級，並非容量規劃。
- 譯文均為草稿，等待審閱。
- 沒有用於起草或裁決變更請求的介面。
- 沒有外掛或應用市場生態。

更多詳情見[比較說明](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md)。更正可透過提交問題並附上來源的方式提出。
