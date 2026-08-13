use crate::reports::utilization::get_total_ticks;
use procfs::{Current, CurrentSI, ProcError};
use std::collections::HashMap;

pub fn print_all_processes() -> Result<String, ProcError> {
    let mut result = String::new();

    result += &format!(
        "{:>5} {:>5} {:>5} {:>7} {:>10} {}\n",
        "PID", "PPID", "STATE", "THREADS", "VmSize(MB)", "NAME"
    );

    for process in procfs::process::all_processes()?.flatten() {
        if let Ok(status) = process.status() {
            result += &format!(
                "{:>5} {:>5} {:>5} {:>7} {:>10} {}\n",
                status.pid,
                status.ppid,
                status.state.chars().next().unwrap_or('X'),
                status.threads,
                (status.vmsize.unwrap_or(0) as f64 * 0.001024).floor(),
                status.name
            );
        }
    }

    Ok(result)
}
struct MyMemProcess {
    pid: i32,
    mem: u64,
    name: String,
}

struct MyCPUProcess {
    pid: i32,
    cpu_time: u64,
    name: String,
}

pub fn print_top_mem_processes() -> Result<String, ProcError> {
    let mut result = String::new();

    let mut mem_snapshot: Vec<MyMemProcess> = Vec::new();

    for process in procfs::process::all_processes()?.flatten() {
        if let Ok(status) = process.status() {
            mem_snapshot.push(MyMemProcess {
                pid: status.pid,
                mem: status.vmrss.unwrap_or(0),
                name: status.name,
            })
        }
    }

    mem_snapshot.sort_unstable_by_key(|a| std::cmp::Reverse(a.mem));

    result += &format!(
        "{:>4} {:>6} {:>10} {}\n",
        "Rank", "PID", "Memory(MB)", "Process"
    );

    for (i, snap) in mem_snapshot.iter().take(5).enumerate() {
        result += &format!(
            "{:>4} {:>6} {:>10} {}\n",
            i + 1,
            snap.pid,
            (snap.mem as f64 * 0.001024).floor(),
            snap.name
        );
    }

    Ok(result)
}

pub fn print_top_cpu_processes() -> Result<String, ProcError> {
    let mut result = String::new();

    let mut cpu_process_map: HashMap<i32, MyCPUProcess> = HashMap::new();
    let mut cpu_snapshot: Vec<MyCPUProcess> = Vec::new();
    let num_cpus = procfs::CpuInfo::current()
        .map(|c| c.num_cores())
        .unwrap_or(1) as f64;

    for process in procfs::process::all_processes()?.flatten() {
        if let Ok(stat) = process.stat() {
            cpu_process_map.insert(
                stat.pid,
                MyCPUProcess {
                    pid: stat.pid,
                    name: stat.comm,
                    cpu_time: stat.utime + stat.stime,
                },
            );
        }
    }
    let cpu_ticks_old = get_total_ticks(&procfs::KernelStats::current()?.total);
    std::thread::sleep(std::time::Duration::from_secs(1));
    for process in procfs::process::all_processes()?.flatten() {
        if let Ok(stat) = process.stat()
            && let Some(prev) = cpu_process_map.get(&stat.pid)
        {
            let curr_ticks = stat.utime + stat.stime;
            cpu_snapshot.push(MyCPUProcess {
                pid: prev.pid,
                name: stat.comm,
                cpu_time: curr_ticks - prev.cpu_time,
            })
        }
    }
    let cpu_ticks_new = get_total_ticks(&procfs::KernelStats::current()?.total);

    cpu_snapshot.sort_unstable_by_key(|a| std::cmp::Reverse(a.cpu_time));
    result += &format!("{:>4} {:>6} {:>8} {}\n", "Rank", "PID", "CPU(%)", "Process");

    for (i, snap) in cpu_snapshot.iter().take(5).enumerate() {
        result += &format!(
            "{:>4} {:>6} {:>6.2}% {}\n",
            i + 1,
            snap.pid,
            100.0 * num_cpus * (snap.cpu_time as f64 / (cpu_ticks_new - cpu_ticks_old)),
            snap.name
        );
    }

    Ok(result)
}
