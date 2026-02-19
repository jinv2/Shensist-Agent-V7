use calamine::{open_workbook, Reader, Xlsx};
use rust_xlsxwriter::{Format, Workbook, Color};
use reqwest::Client;
use serde_json::{json, Value};
// use walkdir::WalkDir; // 暂时注释掉，未使用
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::{Manager, State, Emitter};

pub struct AppState {
    pub is_processing: Arc<Mutex<bool>>,
}

#[tauri::command]
async fn start_analysis(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let is_processing = state.is_processing.clone();
    
    // 检查是否正在处理
    {
        let mut processing = is_processing.lock().unwrap();
        if *processing {
            return Err("数据流正在处理中，请稍候...".to_string());
        }
        *processing = true;
    }

    // 在新线程中执行处理
    let is_processing_clone = is_processing.clone();
    let app_handle_clone = app_handle.clone();
    
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = process_excel_data(&app_handle_clone).await {
                emit_error(&app_handle_clone, &format!("处理失败: {}", e)).await;
            }
            
            // 重置处理状态
            let mut processing = is_processing_clone.lock().unwrap();
            *processing = false;
        });
    });

    Ok("数据流启封成功，处理已开始...".to_string())
}

async fn process_excel_data(app_handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 注意：Linux 路径对中文敏感，必须精确匹配
    let file_path = "/home/mmm/桌面/Shensist_V7_Release/test.xlsx";
    
    // 添加"显影剂"级调试日志
    eprintln!("🔥🔥🔥 [神思庭 Shensist] 核心引擎启动！锁定路径: /home/mmm/桌面/Shensist_V7_Release/test.xlsx");
    
    if std::path::Path::new(file_path).exists() {
        eprintln!("✅ [神思庭 Shensist] 文件存在！正在调用 Calamine 解析...");
        // 这里继续执行原本的 open_workbook 等逻辑
    } else {
        eprintln!("❌ [神思庭 Shensist] 致命错误：路径下找不到 test.xlsx！");
        // 必须返回错误，让前端弹窗
        return Err(format!("文件不存在: {}", file_path).into());
    }
    
    emit_log(app_handle, "🚀 [神思庭 Shensist V7.3 终极版] 四十年文化艺术底蕴 · 智能财务分析系统启动...", "info").await;
    emit_log(app_handle, "⚡ CoBridge 物理写入权限已激活", "success").await;
    
    let client = Client::new();
    // 暂时注释掉原本的路径获取逻辑
    // let data_path = "../data";
    let _data_path = std::path::Path::new(file_path).parent().unwrap_or(std::path::Path::new("."));  // 暂时注释掉原本的路径获取逻辑
    
    // 直接处理指定的单个文件
    emit_log(app_handle, &format!("🔍 正在处理指定文件: {}", file_path), "info").await;
    
    // 1. 创建目标 Excel 工作簿
    emit_log(app_handle, "📊 创建目标财务报表...", "info").await;
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    
    // 设置表头样式
    let header_format = Format::new().set_bold().set_background_color(Color::Silver);
    worksheet.write_with_format(0, 0, "日期", &header_format)?;
    worksheet.write_with_format(0, 1, "金额", &header_format)?;
    worksheet.write_with_format(0, 2, "类型", &header_format)?;
    worksheet.write_with_format(0, 3, "摘要", &header_format)?;

    let mut row_index = 1;
    let mut processed_files = 0;
    let total_files = 1;

    // 直接处理指定的文件，而不是遍历目录
    let path = std::path::Path::new(file_path);
    if path.is_file() && (path.extension().map_or(false, |ext| ext == "xlsx" || ext == "xls")) {
        let file_name = path.file_name().unwrap().to_string_lossy();
        emit_log(app_handle, &format!("📊 正在解析: {}", file_name), "info").await;

        // 使用 calamine 读取 Excel 内容
        let mut workbook_input: Xlsx<_> = open_workbook(file_path)?;
        
        // 动态读取工作表
        let sheet_names = workbook_input.sheet_names();
        eprintln!("🔥🔥🔥 [神思庭 Shensist] 发现工作表: {:?}", sheet_names);
        
        if sheet_names.is_empty() {
            eprintln!("❌ [神思庭 Shensist] 工作簿为空！没有找到任何工作表");
            emit_log(app_handle, "❌ 工作簿为空！没有找到任何工作表", "error").await;
            return Ok(());
        }
        
        let sheet_name = sheet_names.first().unwrap();
        eprintln!("🔥🔥🔥 [神思庭 Shensist] 正在读取工作表: {}", sheet_name);
        emit_log(app_handle, &format!("📊 神思庭 Shensist 正在解析工作表: {}", sheet_name), "info").await;
        
        let mut full_text = String::new();
        
        // 读取第一个工作表
        eprintln!("🔥🔥🔥 [神思庭 Shensist] 正在读取工作表: {}", sheet_name);
        // 注意：这里直接匹配 Ok(range)
        if let Ok(range) = workbook_input.worksheet_range(sheet_name) {
            for (index, row) in range.rows().enumerate().take(5) {
                // 显式指定类型 calamine::Data，防止编译器推断失败
                let row_data: Vec<String> = row.iter().map(|c: &calamine::Data| c.to_string()).collect();
                eprintln!("Row {}: {:?}", index, row_data);
            }
            
            // 读取所有数据
            for row in range.rows() {
                let row_data: Vec<String> = row.iter().map(|c: &calamine::Data| c.to_string()).collect();
                full_text.push_str(&row_data.join(" "));
                full_text.push('\n');
            }
            
            eprintln!("🔥🔥🔥 [神思庭 Shensist] 共读取 {} 行数据", range.rows().count());
        } else {
            eprintln!("❌ [神思庭 Shensist] 无法读取工作表范围！");
            emit_log(app_handle, &format!("❌ 神思庭 Shensist 无法读取工作表: {}", sheet_name), "error").await;
            return Ok(());
        }

        // AI 接口调用（强制同步等待）
        let api_url = "http://127.0.0.1:8080/v1/chat/completions";
        eprintln!("📡 [神思庭] 正在向接口 {} 发送数据流...", api_url);
        eprintln!("🔥🔥🔥 [神思庭 Shensist] 正在调用 AI 接口，等待响应...");
        
        let request_body = json!({
            "model": "/mnt/BigDisk/工程/Qwen2.5-Coder-3B-Instruct-GGUF (Q4_K_M)备份/model.gguf",
            "messages": [
                { "role": "system", "content": "你是一个严谨的财务分析师。请忽略所有无关信息，仅对提供的 Excel 数据进行简要评价。必须直接返回 JSON 格式，不要包含 Markdown 标记。" },
                { "role": "user", "content": format!("请根据以下 Excel 数据，直接输出 1 条简要的财务健康度评价，仅返回 JSON 格式。数据：{}", full_text) }
            ],
            "temperature": 0.3,
            "max_tokens": 500
        });
        
        eprintln!("🔥🔥🔥 [神思庭] 请求体: {}", request_body);
        
        match client.post(api_url)
            .json(&request_body)
            .send()
            .await {
            Ok(res) => {
                eprintln!("🔥🔥🔥 [神思庭 Shensist] AI 接口已响应，状态码: {}", res.status());
                match res.json::<serde_json::Value>().await {
                    Ok(response_json) => {
                        eprintln!("🔥🔥🔥 [神思庭 Shensist] AI 响应解析成功: {}", response_json);
                        let cleaned = response_json["choices"][0]["message"]["content"].as_str().unwrap_or("{}");
                        emit_log(app_handle, "✅ 神思庭 Shensist AI 清洗完成", "success").await;
                        
                        // 假设 cleaned 是 AI 返回的 JSON 字符串
                        let cleaned_val: Value = serde_json::from_str(cleaned).unwrap_or(json!({}));
                        
                        // 2. 将清洗后的数据回填到新 Excel
                        worksheet.write(row_index, 0, cleaned_val["date"].as_str().unwrap_or(""))?;
                        worksheet.write(row_index, 1, cleaned_val["amount"].as_str().unwrap_or(""))?;
                        worksheet.write(row_index, 2, cleaned_val["type"].as_str().unwrap_or(""))?;
                        worksheet.write(row_index, 3, cleaned_val["summary"].as_str().unwrap_or(""))?;
                        
                        row_index += 1;
                        eprintln!("🔥🔥🔥 [神思庭 Shensist] 数据已写入报表");
                    }
                    Err(e) => {
                        eprintln!("❌ [神思庭 Shensist] AI 响应解析失败: {}", e);
                        emit_log(app_handle, &format!("❌ 神思庭 Shensist AI 响应解析失败: {}", e), "error").await;
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ [神思庭 Shensist] AI 接口连接失败: {}", e);
                eprintln!("💡 [神思庭] 诊断：请确保在终端运行了本地模型服务器 (llama-server 或 ollama)。");
                eprintln!("💡 [神思庭] 建议执行：./launch_engine.sh");
                eprintln!("💡 [神思庭] 或手动执行：python3 -m llama_cpp.server --model \"/mnt/BigDisk/工程/Qwen2.5-Coder-3B-Instruct-GGUF (Q4_K_M)备份/qwen2.5-coder-3b-instruct-q4_k_m.gguf\" --port 8080 --n_gpu_layers 33");
                emit_log(app_handle, &format!("❌ 神思庭 Shensist AI 接口连接失败: {}", e), "error").await;
            }
        }
        processed_files += 1;
        
        emit_log(app_handle, &format!("✅ 已处理 {}/{} 个文件", processed_files, total_files), "success").await;
        
        // 添加短暂延迟以显示处理过程
        tokio::time::sleep(Duration::from_millis(500)).await;
    } else {
        emit_log(app_handle, "❌ 神思庭 Shensist 文件格式不支持，请提供 .xlsx 或 .xls 文件", "error").await;
        eprintln!("❌ [神思庭 Shensist] 文件格式不支持");
        return Ok(());
    }

    // 3. 保存最终报表（即使没有数据也保存空表）
    emit_log(app_handle, "💾 正在生成最终报表...", "info").await;
    match workbook.save("../Final_Financial_Report.xlsx") {
        Ok(_) => {
            emit_log(app_handle, "🎯 神思庭 Shensist 标准报表已生成: Final_Financial_Report.xlsx", "success").await;
            eprintln!("✅ [神思庭 Shensist] 报表保存成功");
        }
        Err(e) => {
            emit_log(app_handle, &format!("❌ 神思庭 Shensist 报表保存失败: {}", e), "error").await;
            eprintln!("❌ [神思庭 Shensist] 报表保存失败: {}", e);
        }
    }
    
    emit_log(app_handle, &format!("🎉 神思庭 Shensist 处理完成！共处理 {} 个文件，生成 {} 条记录", total_files, row_index - 1), "success").await;
    eprintln!("🎉 [神思庭 Shensist] 处理完成！共处理 {} 个文件，生成 {} 条记录", total_files, row_index - 1);
    
    // 通知前端按钮状态重置
    let _ = app_handle.emit("processing-complete", ());
    
    // 防止卡死，确保总是返回 Ok(())
    eprintln!("✅ [神思庭 Shensist] 函数正常结束，返回 Ok(())");
    Ok(())
}

async fn emit_log(app_handle: &tauri::AppHandle, message: &str, log_type: &str) {
    let _ = app_handle.emit("console-log", (message, log_type));
}

async fn emit_error(app_handle: &tauri::AppHandle, message: &str) {
    let _ = app_handle.emit("console-log", (message, "error"));
}

#[tauri::command]
fn get_system_info() -> Result<String, String> {
    Ok("神思庭 Shensist V7.3 终极版 - 四十年文化艺术底蕴 · GTX 1050 Ti 加速".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState {
            is_processing: Arc::new(Mutex::new(false)),
        })
        .invoke_handler(tauri::generate_handler![
            start_analysis,
            get_system_info
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
