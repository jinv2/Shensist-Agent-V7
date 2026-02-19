#!/bin/bash

# 神思庭 Shensist V7.3 - 大脑启动脚本
# 自动激活虚拟环境并启动 AI 引擎

echo "🧠 [神思庭] 正在启动大脑..."

# 检查虚拟环境是否存在
if [ ! -d "ai_env" ]; then
    echo "❌ [神思庭] 虚拟环境 ai_env 不存在"
    echo "💡 [神思庭] 请先创建虚拟环境：python3 -m venv ai_env"
    exit 1
fi

# 激活虚拟环境
echo "🔥🔥🔥 [神思庭] 激活虚拟环境..."
source ai_env/bin/activate

# 检查激活是否成功
if [ -z "$VIRTUAL_ENV" ]; then
    echo "❌ [神思庭] 虚拟环境激活失败"
    exit 1
fi

echo "✅ [神思庭] 虚拟环境已激活: $VIRTUAL_ENV"

# 检查模型文件是否存在
MODEL_PATH="/mnt/BigDisk/工程/Qwen2.5-Coder-3B-Instruct-GGUF (Q4_K_M)备份/qwen2.5-coder-3b-instruct-q4_k_m.gguf"
if [ ! -f "$MODEL_PATH" ]; then
    echo "❌ [神思庭] 模型文件不存在: $MODEL_PATH"
    echo "💡 [神思庭] 请检查模型路径是否正确"
    exit 1
fi

echo "✅ [神思庭] 模型文件已找到"

# 杀死可能残留的 8080 端口进程
echo "🔥🔥🔥 [神思庭] 清理端口残留..."
pkill -f "llama_cpp.server" 2>/dev/null
pkill -f "8080" 2>/dev/null
sleep 2

# 启动 AI 引擎
echo "🧠 [神思庭] 正在启动大脑..."
echo "📡 [神思庭] 模型: Qwen2.5-Coder-3B-Instruct"
echo "📡 [神思庭] API 端点: http://127.0.0.1:8080/v1/chat/completions"
echo "📡 [神思庭] GPU 层数: 33 (1050 Ti 最大化)"
echo "📡 [神思庭] 上下文: 4096"

# 在虚拟环境中启动服务器
python3 -m llama_cpp.server \
    --model "$MODEL_PATH" \
    --host 127.0.0.1 \
    --port 8080 \
    --n_gpu_layers 33 \
    --ctx_size 4096 \
    --temperature 0.7 \
    --max_tokens 500

# 如果到这里说明服务器启动失败
echo "❌ [神思庭] 大脑启动失败"
echo "💡 [神思庭] 请检查错误信息并重试"
