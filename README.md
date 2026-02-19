# 🧠 Shensist Agent V7 (神思智能体 V7)

> **四十年文化底蕴 · 智能财务分析系统**
>
> **40 Years of Cultural Heritage · AI Financial Analysis Agent**

[![Powered by Shensist](https://img.shields.io/badge/Powered_by-Shensist-0056D2.svg)](https://shensist.top)
[![Model](https://img.shields.io/badge/Model-Qwen2.5_Coder_3B-green.svg)](https://modelscope.cn/models/qwen/Qwen2.5-Coder-3B-Instruct-GGUF)
[![Platform](https://img.shields.io/badge/Platform-Ubuntu_25_%7C_GTX_1050_Ti-orange.svg)]()

## 📖 项目简介 (Introduction)
**神思智能体 V7** 是一个基于 **本地大语言模型 (Local LLM)** 与 **Rust/Tauri** 核心构建的智能财务分析超个体。
它结合了传统文学的厚度与现代 AI 的算力，能够在完全离线的情况下读取 Excel 财务报表，并生成具备深度的专业评价。

**Shensist Agent V7** is a secure, local AI agent optimized for **NVIDIA GTX 1050 Ti**. It leverages Qwen 2.5 (3B) and a high-performance Rust backend to analyze financial data with zero data leakage.

---

## 📥 模型下载 (Model Download)
⚠️ **注意 / Important**:
由于 GitHub 文件大小限制，本仓库**不包含** AI 模型权重文件。请务必从以下地址下载模型：
The AI model weights are **NOT** included in this repo. Please download them separately:

### 1. 🚀 高速下载 (推荐 / Recommended)
我们推荐使用 ModelScope 魔搭社区进行高速下载：
* **下载地址**: [Qwen2.5-Coder-3B-Instruct-GGUF (ModelScope)](https://modelscope.cn/models/qwen/Qwen2.5-Coder-3B-Instruct-GGUF/files)
* **文件名**: 请寻找 `qwen2.5-coder-3b-instruct-q4_k_m.gguf`

### 2. ☁️ 备用网盘
* *(在此处粘贴您的百度网盘链接，如有)*

---

## 🛠️ 安装与运行 (Installation)

### 1. 克隆仓库 (Clone)
```bash
git clone https://github.com/jinv2/Shensist-Agent-V7.git
cd Shensist-Agent-V7
```

### 2. 放置模型 (Place Model)
下载模型文件后，请将其重命名为 `model.gguf` 并修改 `启动神思.sh` 中的路径，或者直接按照脚本提示放置。

### 3. 一键启动 (Start)
```bash
chmod +x 启动神思.sh
./启动神思.sh
```

---

## 🏗️ 技术架构 (Tech Stack)
* **Core**: Rust (Tauri v2) - 极速安全
* **AI Engine**: llama.cpp (Python Binding) - 适配 1050 Ti
* **Frontend**: Vanilla JS / HTML5 - 轻量级交互
* **OS**: Optimized for Ubuntu 25

---
© 2026 [神思庭 Shensist](https://shensist.top). All Rights Reserved.
