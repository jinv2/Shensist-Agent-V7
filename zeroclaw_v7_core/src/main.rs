use calamine::{open_workbook, Reader, Xlsx};
use rust_xlsxwriter::{Format, Workbook, Color};
use reqwest::Client;
use serde_json::{json, Value};
use walkdir::WalkDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🚀 [神思庭 ZeroClaw V7.3 终极版] 启动...");
    let client = Client::new();
    let data_path = "../data";
    
    // 1. 创建目标 Excel 工作簿
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    
    // 设置表头样式
    let header_format = Format::new().set_bold().set_background_color(Color::Silver);
    worksheet.write_with_format(0, 0, "日期", &header_format)?;
    worksheet.write_with_format(0, 1, "金额", &header_format)?;
    worksheet.write_with_format(0, 2, "类型", &header_format)?;
    worksheet.write_with_format(0, 3, "摘要", &header_format)?;

    let mut row_index = 1;

    for entry in WalkDir::new(data_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && (path.extension().map_or(false, |ext| ext == "xlsx" || ext == "xls")) {
            let file_name = path.file_name().unwrap().to_string_lossy();
            println!("\n📊 正在解析 Excel: {}", file_name);

            // 使用 calamine 读取 Excel 内容
            let mut workbook_input: Xlsx<_> = open_workbook(path)?;
            let mut full_text = String::new();

            // 读取第一个工作表
            if let Some(Ok(range)) = workbook_input.worksheet_range_at(0) {
                for row in range.rows() {
                    let row_data: Vec<String> = row.iter().map(|c| c.to_string()).collect();
                    full_text.push_str(&row_data.join(" "));
                    full_text.push('\n');
                }
            }

            // 调用 AI 进行结构化清洗
            let request_body = json!({
                "model": "shensist-v7-core",
                "stream": false,
                "format": "json",
                "prompt": format!("任务：将以下 Excel 流水行转为标准财务 JSON。\n内容：{}", full_text)
            });

            print!("⚡ AI 推理中 (GTX 1050 Ti)...");
            let res = client.post("http://localhost:11434/api/generate").json(&request_body).send().await?;
            let response_json: Value = res.json().await?;
            let cleaned = response_json["response"].as_str().unwrap_or("{}");

            println!("\n✅ 解析完成:\n{}", cleaned);
            
            // 假设 cleaned 是 AI 返回的 JSON 字符串
            let cleaned_val: Value = serde_json::from_str(cleaned).unwrap_or(json!({}));
            
            // 2. 将清洗后的数据回填到新 Excel
            worksheet.write(row_index, 0, cleaned_val["date"].as_str().unwrap_or(""))?;
            worksheet.write(row_index, 1, cleaned_val["amount"].as_str().unwrap_or(""))?;
            worksheet.write(row_index, 2, cleaned_val["type"].as_str().unwrap_or(""))?;
            worksheet.write(row_index, 3, cleaned_val["summary"].as_str().unwrap_or(""))?;
            
            row_index += 1;
        }
    }

    // 3. 保存最终报表
    workbook.save("../Final_Financial_Report.xlsx")?;
    println!("\n✨ 标准报表已生成至：Shensist_V7_Release/Final_Financial_Report.xlsx");
    Ok(())
}
