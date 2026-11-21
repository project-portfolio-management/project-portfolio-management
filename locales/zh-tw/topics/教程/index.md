# 教程

*在本地電腦上執行演示並檢視示例組合的步驟。*

演示是一份完整的軟體副本，載有虛構組織 Northgate Transport Authority 的資料。所有人員、計劃、數字和日期都是虛構的，資料可以自由更改。

**安全說明。** 演示信任一個已公開的測試金鑰，不得在共享網路上執行，也不得用於真實資料。

## 要求

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) 及 Compose

## 第 1 步：下載程式碼

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## 第 2 步：啟動演示

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

此命令會構建軟體、啟動資料庫、載入虛構組合並啟動介面。首次構建耗時較長。

## 第 3 步：登入

開啟 <http://localhost:3000/zh-001/signin>，保留地址 `example@example.com`，然後按下登入演示的按鈕。不會傳送電子郵件。

首先顯示的頁面是**演示導覽**，其中按組列出每項功能，並附有說明和連結。每個頁面上的黃色橫幅都註明資料是虛構的。

## 第 4 步：檢視組合

在導覽中開啟「計劃」，檢視組合及其專案集和專案。然後開啟組合儀表板。

## 第 5 步：檢視一個專案

透過導覽的交付分組開啟 **North Line interlocking** 專案。檢視其甘特圖、關鍵路徑和掙值。

## 第 6 步：檢視個人工作

開啟「我的工作」和個人看板。當前會話以 Example Person 的身份登入，此人有未完成的任務。

## 第 7 步：檢視產能、啟動控制和報告

開啟產能熱力圖，檢視超額分配的資源。開啟啟動佇列，其中一個專案集因在製品限制而等待。開啟董事報告包，這是釋出後即被凍結的月度摘要。

## 第 8 步：停止演示

此命令會停止所有服務並刪除演示資料：

```sh
podman compose --profile demo down -v
```

## 後續步驟

- [安裝指南](../../../../INSTALL.md)，涵蓋登入和身份提供方
- 面向高階角色的[角色簡報](../../for/)
- [演示指南](../../../../operations/demo.md)
- [功能列表](../功能/index.md)

問題可透過[問題跟蹤器](https://github.com/project-portfolio-management/project-portfolio-management/issues)報告。
