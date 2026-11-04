# 比较

*它与其他工具的关系，供正在判断是否适合的人参考。*

不同的产品解决不同的问题，因此这里不打分。这些说明来自对各项目公开资料的阅读。**没有安装或运行过任何竞品**，而且它们此后都有变化，请查阅其最新文档。

| 项目 | 它是什么 | 本项目的不同之处 |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | 面向非项目经理的项目管理工具 | 本项目还提供组合管理办公室所需的完整进度、成本和治理功能 |
| [Redmine](https://www.redmine.org/) | 可配置的通用工具，可自定义问题类型、状态和角色 | 这里的词汇大多固定在代码中：有自定义字段，但没有按类型的工作流矩阵 |
| Huly Platform（通过其分支 Platform Collective） | 基于多个服务的业务应用框架 | 那是一个平台；这是一个基于 PostgreSQL 的服务 |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | Microsoft Project 的桌面替代品，面向一位经理和一个项目 | 这是一项托管的多用户服务，带有访问控制 |

## 逐个产品比较

阅读每个产品的页面（英文）：[Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md)。

## 与您现有的工具配合使用

- **Jira 和 Asana：** CSV 导入，提供预览，并说明所有未导入内容的原因。
- **Microsoft Project：** 导入和导出 MSPDI XML，涵盖任务、大纲、日期、工期、约束、依赖关系和日历。不包括任务负责人、资源和基线，文件中会说明这一点。

## 有意为之的不同之处

- **它会说明原因。** 缺失的数字显示为“无预测，因为……”，绝不把猜测当作数字。
- **重复检测可解释，** 并适用于所有类型的计划。
- **默认安全。** 安全是仓库的一部分，而不是额外的东西。
- **明确的工期。** 每个工期都以日历日或工作日表示。
- **诚实的状态。** 待办清单说明了每一项不做什么。

## 不足之处

- 没有移动应用，也没有桌面应用。
- 没有实时协同编辑。
- 只有一位维护者，代码库还很年轻，没有长期的生产历史，也没有第三方审计。
- 已发布负载测试，但只在一台机器上进行，因此只是一个数量级，而不是容量规划。
- 译文均为草稿，等待审阅。
- 还没有用于起草或裁决变更请求的界面。
- 没有成熟产品那样的插件或应用市场生态。

如需更完整的内容，请阅读[完整的比较说明](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md)。如果发现错误，请提交问题并附上来源。
