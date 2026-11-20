# 教程

*在本地电脑上运行演示并查看示例组合的步骤。*

演示是一份完整的软件副本，载有虚构组织 Northgate Transport Authority 的数据。所有人员、计划、数字和日期都是虚构的，数据可以自由更改。

**安全说明。** 演示信任一个已公开的测试密钥，不得在共享网络上运行，也不得用于真实数据。

## 要求

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) 及 Compose

## 第 1 步：下载代码

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## 第 2 步：启动演示

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

此命令会构建软件、启动数据库、载入虚构组合并启动界面。首次构建耗时较长。

## 第 3 步：登录

打开 <http://localhost:3000/zh-001/signin>，保留地址 `example@example.com`，然后按下登录演示的按钮。不会发送电子邮件。

首先显示的页面是**演示导览**，其中按组列出每项功能，并附有说明和链接。每个页面上的黄色横幅都注明数据是虚构的。

## 第 4 步：查看组合

在导览中打开“计划”，查看组合及其项目集和项目。然后打开组合仪表板。

## 第 5 步：查看一个项目

通过导览的交付分组打开 **North Line interlocking** 项目。查看其甘特图、关键路径和挣值。

## 第 6 步：查看个人工作

打开“我的工作”和个人看板。当前会话以 Example Person 的身份登录，此人有未完成的任务。

## 第 7 步：查看产能、启动控制和报告

打开产能热力图，查看超额分配的资源。打开启动队列，其中一个项目集因在制品限制而等待。打开董事报告包，这是发布后即被冻结的月度摘要。

## 第 8 步：停止演示

此命令会停止所有服务并删除演示数据：

```sh
podman compose --profile demo down -v
```

## 后续步骤

- [安装指南](../../../../INSTALL.md)，涵盖登录和身份提供方
- 面向高级角色的[角色简报](../../for/)
- [演示指南](../../../../operations/demo.md)
- [功能列表](../功能/index.md)

问题可通过[问题跟踪器](https://github.com/project-portfolio-management/project-portfolio-management/issues)报告。
