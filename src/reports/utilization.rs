use procfs::{CpuTime, CurrentSI};
pub fn get_total_ticks(t: &CpuTime) -> f64 {
    (t.user
        + t.nice
        + t.system
        + t.idle
        + t.iowait.unwrap_or(0)
        + t.irq.unwrap_or(0)
        + t.softirq.unwrap_or(0)
        + t.steal.unwrap_or(0)) as f64
}

pub fn get_idle_ticks(t: &CpuTime) -> f64 {
    (t.idle + t.iowait.unwrap_or(0)) as f64
}

pub fn print_cpu_utilization() -> Result<String, procfs::ProcError> {
    let mut result = String::new();

    let cpu_util_old = procfs::KernelStats::current()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    let cpu_util_new = procfs::KernelStats::current()?;

    let time_old = cpu_util_old.total;
    let time_new = cpu_util_new.total;

    let time_utilized = get_total_ticks(&time_new) - get_total_ticks(&time_old);
    let idle_utilized = get_idle_ticks(&time_new) - get_idle_ticks(&time_old);

    if time_utilized > 0.0 {
        let cpu_usage = (time_utilized - idle_utilized) / time_utilized;
        result += &format!("CPU Usage : {:.1}%\n", cpu_usage * 100.0);
    } else {
        result += &format!("CPU Usage : 0.0%\n");
    }

    Ok(result)
}
