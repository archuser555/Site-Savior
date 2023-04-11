// To get system info, for statistics :)
use std::process::Command;

// return a touple, total and used, in GB
pub fn mem_info() -> (String, String) {
    let output = Command::new("free")
    .arg("-h")
    .output()
    .expect("mem err");

    let output_str = String::from_utf8_lossy(&output.stdout);

    let mut total_mem = String::new();
    let mut used_mem = String::new();

    for line in output_str.lines() {
        if line.starts_with("Mem:") {
            let cols: Vec<&str> = line.split_whitespace().collect();
            total_mem = cols[1].to_string();
            used_mem = cols[2].to_string();
        }
    }

    (used_mem, total_mem)
}

pub fn cpu_info() -> String {
    String::from_utf8_lossy(
        &Command::new("sh")
            .arg("-c")
            .arg("cat /proc/cpuinfo | grep \"cpu MHz\" | awk '{sum += $4} END {printf(\"%.2f\\n\", sum/NR)}'")
            .output()
            .expect("Failed to execute command")
            .stdout
    ).to_string()
}

pub fn uptime() -> String {
    let output = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("uptime command failed");
    return String::from_utf8_lossy(&output.stdout).into_owned();
}

// return used disk and total also, in GigaBhad :)
pub fn disk_info() -> String {
    let output = String::from_utf8_lossy(
    &Command::new("sh")
        .arg("-c")
        .arg("inxi -D -c | grep \"Local Storage\"")
        .output()
        .expect("disk err")
        .stdout
    ).to_string();
    // delete weird shit
    output.replace("Local Storage: ", "")
}