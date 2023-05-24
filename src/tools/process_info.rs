use crate::Error;

use regex;

use tokio::fs::File;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;

pub async fn get_process_info(pid: u32) -> Result<String, Error> {

    let file = File::open(format!("/proc/{}/status", pid)).await?;

    let buf = BufReader::new(file);

    let mut lines = buf.lines();

    let mut output: String = String::new();


    while let Some(line) = lines.next_line().await? {
        if line.starts_with("VmPeak:") {
            let (mem_kb, mem_gb) = get_mem_info(&line).await?;
            output.push_str(format!("Peak memory usage: \nKB: {} | GB: {:.2}\n", mem_kb, mem_gb).as_str());
        }
        if line.starts_with("VmSize:") {
            let (mem_kb, mem_gb) = get_mem_info(&line).await?;
            output.push_str(format!("Memory usage: \nKB: {} | GB: {:.2}\n", mem_kb, mem_gb).as_str());
        }
    };

    Ok(output)
}

// Gets the numbers out of the line, formats them into floats and calculates the usage in GB too
async fn get_mem_info(line: &str) -> Result<(f32, f32), Error> {
    let re = regex::Regex::new("[\\d]+").unwrap();

    let tmp = re.find(&line).unwrap().as_str();
    let mem_kb: f32 = tmp.parse::<f32>().unwrap();
    let mem_gb = mem_kb / (1024.0 * 1024.0);

    Ok((mem_kb, mem_gb))
}