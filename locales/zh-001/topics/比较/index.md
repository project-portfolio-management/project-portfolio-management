# 比较

*概述本软件与其他产品的关系。*

下列产品用途各不相同，不作评分。这些说明基于各项目的公开资料。**未安装或运行过任何其他产品**，各产品此后可能已有变化。作出决定前应查阅产品的最新文档。

| 项目 | 它是什么 | 本项目的不同之处 |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | 面向非项目经理的项目管理工具 | 本项目还包括组合管理办公室使用的进度、成本和治理功能 |
| [Redmine](https://www.redmine.org/) | 可配置的通用工具，由组织定义问题类型、状态和角色 | 这里的词汇大多固定在代码中。提供自定义字段，但没有按类型的工作流矩阵 |
| Huly Platform（通过其分支 Platform Collective） | 基于多个服务的业务应用框架 | Huly 是多服务平台。本项目是一个基于 PostgreSQL 的服务 |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Microsoft Project 的桌面替代品，面向一位经理和一个项目 | 本项目是一项托管的多用户服务，带有访问控制 |

## 产品比较

各产品比较页面（英文）：[Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md)。

## 互操作性

- **Jira 和 Asana：** CSV 导入，提供预览，并说明所有未导入内容的原因。
- **Microsoft Project：** 导入和导出 MSPDI XML，涵盖任务、大纲、日期、工期、约束、依赖关系和日历。任务负责人、资源和基线不包括在内，文件中会记录这一点。

## 设计差异

- **说明原因。** 缺失的数字附带原因，例如“无预测，因为……”。
- **可解释的重复检测。** 匹配适用于所有类型的计划，并展示证据。
- **安全默认设置。** 默认启用登录，仓库包含安全资料。
- **明确的工期。** 每个工期都注明日历日或工作日。
- **记录局限。** 待办清单记录了每一项的局限。

## 已知局限

- 没有移动应用或桌面应用。
- 没有实时协同编辑。
- 只有一位维护者，生产历史有限，没有第三方审计。
- 已发布的负载测试在一台机器上运行，只表示数量级，并非容量规划。
- 译文均为草稿，等待审阅。
- 没有用于起草或裁决变更请求的界面。
- 没有插件或应用市场生态。

更多详情见[比较说明](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md)。更正可通过提交问题并附上来源的方式提出。
