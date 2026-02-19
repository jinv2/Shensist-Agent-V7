#!/bin/bash

# ==========================================
# 🧠 天算AI · 神思智能体 V7 (Shensist Agent)
# 🚀 一键启封脚本 | 适配 Ubuntu 25 + GTX 1050 Ti
# ==========================================

# 1. 定义核心路径
PROJECT_DIR="$HOME/桌面/Shensist_V7_Release"
MODEL_PATH="/mnt/BigDisk/工程/Qwen2.5-Coder-3B-Instruct-GGUF (Q4_K_M)备份/model.gguf"
VENV_ACTIVATE="$PROJECT_DIR/ai_env/bin/activate"

# 2. 颜色定义
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}==========================================${NC}"
echo -e "${BLUE}   🧠 神思庭 (Shensist) · 正在唤醒硅基大脑...${NC}"
echo -e "${BLUE}==========================================${NC}"

# 3. 检查并清理旧进程 (防止端口冲突)
if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null ; then
    echo -e "${RED}⚠️  检测到 8080 端口被占用，正在清理...${NC}"
    kill $(lsof -Pi :8080 -sTCP:LISTEN -t)
    sleep 1
fi

# 4. 启动 AI 算力引擎 (后台运行)
echo -e "${GREEN}🚀 [1/2] 正在加载 Qwen 3B 模型至 GTX 1050 Ti...${NC}"
source "$VENV_ACTIVATE"

# 启动命令 (后台运行 &)
python3 -m llama_cpp.server \
  --model "$MODEL_PATH" \
  --host 127.0.0.1 \
  --port 8080 \
  --n_gpu_layers 33 \
  > "$PROJECT_DIR/brain.log" 2>&1 &

# 获取 AI 进程 ID，用于稍后关闭
AI_PID=$!

# 5. 等待 AI 引擎就绪
echo -e "⏳ 等待 AI 神经网络建立连接 (约需 5 秒)..."
sleep 5

# 检查是否启动成功
if ps -p $AI_PID > /dev/null; then
    echo -e "${GREEN}✅ 大脑已上线！(PID: $AI_PID)${NC}"
else
    echo -e "${RED}❌ 大脑启动失败，请查看 brain.log 日志。${NC}"
    exit 1
fi

# 6. 启动前端躯干 (Tauri)
echo -e "${GREEN}🚀 [2/2] 正在启动神思智能体 V7 界面...${NC}"
echo -e "${BLUE}💡 提示：关闭软件窗口后，AI 引擎将自动休眠。${NC}"

# 切换回项目目录并运行
cd "$PROJECT_DIR"
cargo tauri dev

# 7. 退出时的清理工作
# 当 Tauri 关闭后，脚本会执行到这里
echo -e "${BLUE}👋 正在关闭神思智能体...${NC}"
kill $AI_PID
echo -e "${GREEN}✅ AI 算力已释放。期待下一次灵感涌现。${NC}"
