use procfs::Current;

fn format_duration(d: &std::time::Duration) -> (i32, i32, i32) {
    let days = d.div_duration_f32(std::time::Duration::from_hours(24)) as i32;
    let hours = (d.div_duration_f32(std::time::Duration::from_hours(1)) as i32) % 24;
    let minutes = (d.div_duration_f32(std::time::Duration::from_mins(1)) as i32) % 60;

    (days, hours, minutes)
}
pub fn print_uptime() -> Result<(), procfs::ProcError> {
    let uptime = procfs::Uptime::current()?;
    let formatted_uptime = format_duration(&uptime.uptime_duration());
    println!(
        "System Uptime : {} Days {} Hours {} Minutes",
        formatted_uptime.0, formatted_uptime.1, formatted_uptime.2
    );

    Ok(())
}

pub fn print_loadavg() -> Result<(), procfs::ProcError> {
    let loadavg = procfs::LoadAverage::current()?;

    println!("Load Average");
    println!("{:>3} min : {}", 1, loadavg.one);
    println!("{:>3} min : {}", 5, loadavg.five);
    println!("{:>3} min : {}", 15, loadavg.fifteen);

    Ok(())
}
