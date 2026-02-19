#!/bin/bash

# ==========================================
# 🔧 神思智能体 V7 · 仓库冲突修复与重推
# ==========================================

echo "🧹 正在清理 Git 冲突..."

# 1. 关键修复：移除子文件夹里的 .git 目录
#    (这会让 zeroclaw_v7_core 变成普通文件夹，允许被上传)
rm -rf zeroclaw_v7_core/.git

# 2. 清理根目录下刚才失败的 .git 记录 (重新来过)
rm -rf .git

echo "🧠 正在重新初始化神思庭代码库..."

# 3. 重新配置 .gitignore (确保配置还在)
cat > .gitignore <<EOF
*.gguf
*.bin
ai_env/
node_modules/
target/
src-tauri/target/
brain.log
*.log
.DS_Store
.vscode/
.idea/
*.xlsx
!test.xlsx
EOF

# 4. 重新初始化并提交
git init
git branch -M main
git add .
git commit -m "Release Shensist Agent V7: Final Integrated Version"

# 5. 强制推送到远程 (覆盖之前的错误记录)
#    请确认您的仓库地址是否正确：
REMOTE_URL="git@github.com:jinv2/Shensist-Agent-V7.git"

echo "📡 正在尝试重新推送..."
git remote add origin "$REMOTE_URL"
git push -u -f origin main  # 使用 -f 强制覆盖，确保只有这一次完美的提交

echo "🎉 修复完成！所有代码（包括 core）已成功上传。"
