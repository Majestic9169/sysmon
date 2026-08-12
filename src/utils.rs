use procfs::CpuTime;

pub fn print_break() {
    println!("--------------------------------------------------------------");
}
pub fn print_header(num: i32, header: &str) {
    println!("[{}] {}", num, header);
    print_break();
}

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

pub fn format_duration(d: &std::time::Duration) -> (i32, i32, i32) {
    let days = d.div_duration_f32(std::time::Duration::from_hours(24)) as i32;
    let hours = (d.div_duration_f32(std::time::Duration::from_hours(1)) as i32) % 24;
    let minutes = (d.div_duration_f32(std::time::Duration::from_mins(1)) as i32) % 60;

    (days, hours, minutes)
}
