# 教程

*在您自己的电脑上运行演示，浏览一个完整的组合。*

您将启动一份完整的软件副本，其中填充了一个名为 Northgate Transport Authority 的虚构组织。所有人员、计划、资金和日期都是虚构的，您可以随意更改。

**请只在您自己的电脑上运行。** 演示信任一个已公开的测试密钥，因此绝不要把它放到共享网络上，也不要用于真实数据。

## 您需要什么

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) 及 Compose

## 第 1 步：获取代码

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## 第 2 步：启动演示

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

这会构建软件、启动数据库、填入虚构组合并启动界面。第一次构建需要一些时间。

## 第 3 步：登录

打开 <http://localhost:3000/zh-001/signin>，保留地址 `example@example.com`，然后按下登录演示的按钮。不会发送电子邮件。

您会进入**演示导览**，其中按组列出每项功能，并附有查看要点和链接。每个页面上的黄色横幅都会提醒您数据是虚构的。

## 第 4 步：纵览组合

在导览中打开“计划”，查看组合及其项目集和项目，然后打开组合仪表板。请注意，任何计划都可以包含任何其他计划。

## 第 5 步：跟进一个项目

通过导览的交付分组打开 **North Line interlocking** 项目。查看它的甘特图，然后是关键路径，再然后是挣值。

## 第 6 步：查看您自己的工作

打开“我的工作”和您的看板。您以 Example Person 的身份登录，此人有一些未完成的任务。移动一张卡片，看看它如何工作。

## 第 7 步：看它如何如实发声

打开产能热力图，查看谁的工作量超额。然后打开启动队列，其中一个项目集被在制品限制拒绝，正在等待。最后打开董事报告包，这是发布后即被冻结的月度摘要。

## 第 8 步：停止并清理

这会停止所有内容并删除演示数据：

```sh
podman compose --profile demo down -v
```

## 接下来去哪里

- [正式安装](../../../../INSTALL.md)，配置登录和您自己的身份提供方
- 阅读面向高级角色的[角色简报](../../for/)
- 阅读[完整的演示指南](../../../../operations/demo.md)
- [查看全部功能](../功能/index.md)

遇到问题？请[提交问题](https://github.com/project-portfolio-management/project-portfolio-management/issues)。
