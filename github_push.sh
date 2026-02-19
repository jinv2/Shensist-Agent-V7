#!/bin/bash

# ==========================================
# 🚀 神思智能体 V7 · GitHub 发布脚本
# ==========================================

# --- 1. 配置您的远程仓库地址 (请确认此处！) ---
REMOTE_URL="git@github.com:jinv2/Shensist-Agent-V7.git"

echo "🧠 正在初始化神思庭代码库..."

# --- 2. 智能清理：创建 .gitignore (防止大文件卡死) ---
# 这一步至关重要，它会把几 GB 的模型和编译文件挡在门外
cat > .gitignore <<EOF
# 忽略大模型文件
*.gguf
*.bin

# 忽略环境与依赖 (让别人下载代码后自己重建)
ai_env/
node_modules/
target/
src-tauri/target/

# 忽略日志与临时文件
brain.log
*.log
.DS_Store
.vscode/
.idea/

# 忽略本地生成的 Excel 结果
*.xlsx
!test.xlsx
EOF

echo "✅ .gitignore 配置完成，已隔离 model.gguf 和 ai_env"

# --- 3. Git 初始化与提交 ---
git init
git branch -M main
git add .
git commit -m "Release Shensist Agent V7: Integrated Local LLM with Rust/Tauri Core"

# --- 4. 推送 ---
echo "📡 正在通过 SSH 推送到 GitHub..."
git remote add origin "$REMOTE_URL" 2>/dev/null || git remote set-url origin "$REMOTE_URL"
git push -u origin main

echo "🎉 发布成功！项目已上线。"
echo "🔗 仓库地址: https://github.com/jinv2/Shensist-Agent-V7"
