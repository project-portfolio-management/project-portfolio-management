# チュートリアル

*デモンストレーションをローカルのコンピューターで動かし、サンプルのポートフォリオを確認する手順。*

デモンストレーションは、ソフトウェアの完全なコピーに、架空の組織「ノースゲート交通局」のデータを読み込んだものです。人物、計画、数値、日付はすべて架空で、データは自由に変更できます。

**セキュリティ上の注意。** デモンストレーションは、公開されているテスト用の鍵を信頼します。共有ネットワークで実行したり、実際のデータで使ったりしてはいけません。

## 必要なもの

- [Git](https://git-scm.com/)
- Compose 付きの [Podman](https://podman.io/)

## 手順 1: コードをダウンロードする

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## 手順 2: デモンストレーションを起動する

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

このコマンドは、ソフトウェアをビルドし、データベースを起動し、架空のポートフォリオを読み込み、インターフェースを起動します。最初のビルドは、2 回目以降より時間がかかります。

## 手順 3: サインインする

<http://localhost:3000/ja-jp/signin> を開き、アドレスを `example@example.com` のままにして **Sign in to the demonstration** を押します。メールは送信されません。

最初のページは**デモンストレーションツアー**で、すべての機能がグループ別に、説明とリンク付きで一覧されています。すべてのページの黄色いバナーが、データが架空であることを示しています。

## 手順 4: ポートフォリオを確認する

ツアーから **Plans** を開き、ポートフォリオ、そのプログラム、プロジェクトを表示します。続いて **Portfolio dashboard** を開きます。

## 手順 5: 一つのプロジェクトを確認する

ツアーのデリバリーのグループから、**North Line interlocking** プロジェクトを開きます。その **Gantt** チャート、**Critical path**、**Earned value** を確認します。

## 手順 6: 個人の作業を確認する

**My work** と **My kanban board** を開きます。セッションは Example Person としてサインインしており、未完了のタスクがあります。

## 手順 7: キャパシティ、開始管理、レポートを確認する

**Capacity heatmap** を開いて、過剰に割り当てられたリソースを確認します。**Start queue** を開くと、仕掛かり作業の上限により一つのプログラムが保留されています。**Director packs** を開きます。これは月次の要約で、発行されると凍結されます。

## 手順 8: デモンストレーションを停止する

このコマンドは、すべてのサービスを停止し、デモンストレーションのデータを削除します。

```sh
podman compose --profile demo down -v
```

## 次のステップ

- [インストールガイド](../../../../INSTALL.md)（ログインとアイデンティティプロバイダーを扱います）
- 上級の役割向けの[役割別ブリーフ](../../for/)
- [デモンストレーションガイド](../../../../operations/demo.md)
- [機能一覧](../機能/index.md)

問題は[課題トラッカー](https://github.com/project-portfolio-management/project-portfolio-management/issues)から報告できます。
