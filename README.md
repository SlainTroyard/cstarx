# CStarX 代码转译工具

## 简介
CStarX 是一款专为开发者设计的代码转译插件，它的核心目标是实现编程语言间代码的自动转换（该版本实现了 C/C++ 到 Rust 的转换）。本工具基于遗传算法与大模型提示工程，结合了 GLM4 模型 API 与传统转译工具 c2rust 的能力。这种方法论不仅提升了转译的准确性，也拓宽了应用的可能性，为基于大模型的代码转译领域带来了新的视角和方法。

## 环境依赖
**请按照以下步骤配置 CStarX 的运行环境：**

```bash
// 安装或更新必要的 Python 包
pip install zhipuai
pip install scan-build
// 通过 cargo 安装最新的 c2rust
cargo install --git https://github.com/immunant/c2rust.git c2rust
```

*对于直接下载源代码的用户：可以直接执行本项目中的install_env.sh脚本。*

*注意：本插件默认用户已配置 pip，python3，rust，cargo 和 gcc 等 C/C++、Rust、Python 开发必备工具。本工具的默认开发与运行环境均为基于 Linux 内核的 Ubuntu 系统。*

### API密钥到期通知
目前示例中使用的 API 密钥为 cstar 团队成员的个人 API ，该 API 将在 2024 年 4 月 29 日后随时面临过期或限额的问题。如果您想继续研究，请使用自己的密钥。

## 使用说明
1. 将智谱 API 密钥粘贴到 "Enter your zuipu API key..." 输入框中。
2. 点击 "ReadAPIkey" 按钮来保存您输入的 API 密钥。
3. 在输入框中输入您想要转译的代码。
4. 点击 "Translate" 按钮后，等待程序运行。
5. 结果将显示在输出框中。

## 技术路线
CStarX 基于 LLM(ChatGLM) 与 c2rust , 实现了 C/C++ 到 Rust 的代码转译。本工具通过调用智谱清言 GLM4 模型 API ，采用**代码解释-翻译的双段式 PE 规则**实现代码转译。

CStarX 基于**遗传算法的变异规则，异步变换模型的 temperature value 与 top_p value**，在保证模型稳定性的同时，释放模型的开拓性。CStarX 采用的**并行机制**降低了模型输出的时间复杂度。并行线程数默认值为 *5* ，该值为智谱清言免费级（初级）用户的api并行数。
此外，CStarX 集成了经典的 C 到 Rust 转译工具 **c2rust** ，以提升输出可信度。

目前，我们还在探索更为复杂的优化策略和提示工程（PE）模式。由于受到时间和资源的限制，这些高级功能的开发尚未完成。我们期待在未来的版本中，能够将这些高级特性呈现给用户，以进一步提升工具的性能和应用范围。

## 开发计划
我们欢迎vivo方提供更长久的大模型API与合适的实验平台。目前，更复杂的优化策略与PE模式处于开发中，敬请期待未来的更新。

## 联系方式
如果您有任何问题或建议，请通过以下邮箱联系我们：xfliu@whu.edu.cn 。

