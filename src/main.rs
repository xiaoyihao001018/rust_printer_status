use serde::Deserialize;
use std::result::Result;
use wmi::{COMLibrary, WMIConnection};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct PrintJob {
    caption: String,
    job_status: Option<String>,
    status: Option<String>,
    status_mask: Option<i32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Printer {
    name: String,
    work_offline: bool,
    status: Option<i32>,
    detected_error_state: Option<i32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置控制台标题
    println!("打印机状态监控程序");
    println!("==================\n");

    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let print_jobs: Vec<PrintJob> = wmi_con.raw_query("SELECT * FROM Win32_PrintJob")?;
    println!("打印队列:");
    for job in &print_jobs {
        println!("任务: {}", job.caption);
        if let Some(status) = &job.job_status {
            println!("任务状态: {}", status);
        }
        if let Some(status) = &job.status {
            println!("详细状态: {}", status);
        }
        if let Some(mask) = job.status_mask {
            if mask & 0x1 != 0 {
                println!("状态: 暂停");
            }
        }
        println!("");
    }

    let printers: Vec<Printer> = wmi_con.raw_query("SELECT * FROM Win32_Printer")?;
    println!("\n打印机状态:");
    for printer in &printers {
        println!("打印机名称: {}", printer.name);
        
        if printer.work_offline {
            println!("状态: 离线");
            continue;
        }
        
        if let Some(status) = printer.status {
            match status {
                1 => println!("状态: 其他"),
                2 => println!("状态: 未知"),
                3 => println!("状态: 空闲"),
                4 => println!("状态: 打印中"),
                5 => println!("状态: 警告"),
                6 => println!("状态: 测试中"),
                7 => println!("状态: 不可用"),
                _ => println!("状态: 未定义状态"),
            }
        }

        if let Some(error_state) = printer.detected_error_state {
            if error_state & 0x4 != 0 {
                println!("警告: 缺纸!");
            }
        }
    }

    Ok(())
}
