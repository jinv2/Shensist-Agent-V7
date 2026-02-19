#!/bin/bash

# 神思庭 Shensist V7.3 - AI 引擎点火脚本
# 自动启动 llama.cpp 服务器以提供 8080 端口算力流

echo "🔥🔥🔥 [神思庭 Shensist] 正在启动 AI 引擎..."

# 检查 Python 环境
echo "📡 [神思庭] 检查 Python 环境..."
if ! python3 -c "import llama_cpp" 2>/dev/null; then
    echo "❌ [神思庭] llama-cpp-python 未安装，正在安装..."
    pip install llama-cpp-python[server] --user --break-system-packages
    if [ $? -ne 0 ]; then
        echo "❌ [神思庭] 安装失败，尝试使用 pipx..."
        pipx install llama-cpp-python[server]
    fi
else
    echo "✅ [神思庭] llama-cpp-python 已安装"
fi

# 检查模型文件是否存在
MODEL_PATH="/mnt/BigDisk/工程/Qwen2.5-Coder-3B-Instruct-GGUF (Q4_K_M)备份/qwen2.5-coder-3b-instruct-q4_k_m.gguf"
if [ ! -f "$MODEL_PATH" ]; then
    echo "❌ [神思庭] 模型文件不存在: $MODEL_PATH"
    exit 1
fi

echo "✅ [神思庭] 模型文件已找到: $MODEL_PATH"

# 检查端口是否被占用
if lsof -i :8080 >/dev/null 2>&1; then
    echo "❌ [神思庭] 端口 8080 已被占用，正在清理..."
    pkill -f "llama_cpp.server" 2>/dev/null
    sleep 2
fi

# 启动 llama.cpp 服务器
echo "🚀 [神思庭] 正在启动 llama.cpp 服务器..."
echo "📡 [神思庭] 模型: Qwen2.5-Coder-3B-Instruct"
echo "📡 [神思庭] 端口: 8080"
echo "📡 [神思庭] GPU 层数: 33 (最大化 1050 Ti 显存)"
echo "📡 [神思庭] 主机: 127.0.0.1"

# 使用 nohup 后台运行，输出到日志文件
nohup python3 -m llama_cpp.server \
    --model "$MODEL_PATH" \
    --host 127.0.0.1 \
    --port 8080 \
    --n_gpu_layers 33 \
    --ctx_size 4096 \
    --temperature 0.7 \
    --max_tokens 500 \
    > /tmp/shensist_ai.log 2>&1 &

SERVER_PID=$!
echo "✅ [神思庭] 服务器已启动，PID: $SERVER_PID"
echo "📡 [神思庭] 日志文件: /tmp/shensist_ai.log"
echo "📡 [神思庭] 等待服务器就绪..."

# 等待服务器启动
sleep 5

# 检查服务器是否正常运行
if ps -p $SERVER_PID > /dev/null; then
    echo "🎉 [神思庭] AI 引擎点火成功！"
    echo "📡 [神思庭] API 端点: http://127.0.0.1:8080/completion"
    echo "📡 [神思庭] 可以开始使用神思庭 Shensist V7.3 系统"
    
    # 测试连接
    echo "📡 [神思庭] 正在测试连接..."
    if curl -s http://127.0.0.1:8080/completion > /dev/null 2>&1; then
        echo "✅ [神思庭] 连接测试成功"
    else
        echo "⚠️ [神思庭] 连接测试失败，但服务器正在运行"
    fi
else
    echo "❌ [神思庭] 服务器启动失败"
    echo "📡 [神思庭] 请检查日志: tail -f /tmp/shensist_ai.log"
    exit 1
fi

echo ""
echo "🔥🔥🔥 [神思庭 Shensist] 四十年文化艺术底蕴 · 智能财务分析系统已就绪"
echo "🔥🔥🔥 [神思庭] 现在可以运行 cargo tauri dev 启动前端"
