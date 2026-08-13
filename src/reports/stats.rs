use procfs::Current;

fn format_duration(d: &std::time::Duration) -> (i32, i32, i32) {
    let days = d.div_duration_f32(std::time::Duration::from_hours(24)) as i32;
    let hours = (d.div_duration_f32(std::time::Duration::from_hours(1)) as i32) % 24;
    let minutes = (d.div_duration_f32(std::time::Duration::from_mins(1)) as i32) % 60;

    (days, hours, minutes)
}
pub fn print_uptime() -> Result<String, procfs::ProcError> {
    let uptime = procfs::Uptime::current()?;
    let formatted_uptime = format_duration(&uptime.uptime_duration());
    let result = format!(
        "System Uptime : {} Days {} Hours {} Minutes\n",
        formatted_uptime.0, formatted_uptime.1, formatted_uptime.2
    );

    Ok(result)
}

pub fn print_loadavg() -> Result<String, procfs::ProcError> {
    let mut result = String::new();

    let loadavg = procfs::LoadAverage::current()?;

    result += &format!("Load Average\n");
    result += &format!("{:>3} min : {}\n", 1, loadavg.one);
    result += &format!("{:>3} min : {}\n", 5, loadavg.five);
    result += &format!("{:>3} min : {}\n", 15, loadavg.fifteen);

    Ok(result)
}
