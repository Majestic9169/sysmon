use crate::utils;
use crate::utils::get_idle_ticks;
use crate::utils::get_total_ticks;
use crate::utils::print_break;
use crate::utils::print_header;
use procfs::Current;
use procfs::CurrentSI;

pub fn print_report() -> Result<(), procfs::ProcError> {
    println!("==============================================================");
    println!("              PROC MONITOR - SYSTEM INFORMATION               ");
    println!("==============================================================");

    print_header(1, "CPU & MEMORY INFORMATION");
    print_cpu_info()?;
    print_mem_info()?;
    print_break();

    print_header(2, "CPU UTILIZATION & SYSTEM STATISTICS");
    print_cpu_utilization()?;
    print_uptime()?;
    print_loadavg()?;
    print_break();

    Ok(())
}

fn print_cpu_info() -> Result<(), procfs::ProcError> {
    let cpu = procfs::CpuInfo::current()?;
    let logical_processors = cpu.num_cores();
    let model_name = cpu
        .model_name(0)
        .ok_or(procfs::ProcError::Other(String::from(
            "Missing CPU model name",
        )))?;

    println!("CPU Model : {}", model_name);
    println!("Logical CPUs : {}", logical_processors);

    Ok(())
}

fn print_mem_info() -> Result<(), procfs::ProcError> {
    let mem = procfs::Meminfo::current()?;

    let avail_mem = mem
        .mem_available
        .ok_or(procfs::ProcError::Other(String::from(
            "Missing Available Memory",
        )))?;

    println!(
        "Total Memory : {}",
        human_memsize::human_size(mem.mem_total)
    );
    println!(
        "Available Memory : {}",
        human_memsize::human_size(avail_mem)
    );
    println!("Free Memory : {}", human_memsize::human_size(mem.mem_free));

    println!(
        "Memory Usage : {:.1}%",
        100f64 * (1f64 - (avail_mem as f64 / mem.mem_total as f64))
    );

    Ok(())
}

fn print_cpu_utilization() -> Result<(), procfs::ProcError> {
    let cpu_util_old = procfs::KernelStats::current()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    let cpu_util_new = procfs::KernelStats::current()?;

    let time_old = cpu_util_old.total;
    let time_new = cpu_util_new.total;

    let time_utilized = get_total_ticks(&time_new) - get_total_ticks(&time_old);
    let idle_utilized = get_idle_ticks(&time_new) - get_idle_ticks(&time_old);

    if time_utilized > 0.0 {
        let cpu_usage = (time_utilized - idle_utilized) / time_utilized;
        println!("CPU Usage : {:.1}%", cpu_usage * 100.0);
    } else {
        println!("CPU Usage : 0.0%");
    }

    Ok(())
}

fn print_uptime() -> Result<(), procfs::ProcError> {
    let uptime = procfs::Uptime::current()?;
    let formatted_uptime = utils::format_duration(&uptime.uptime_duration());
    println!(
        "System Uptime : {} Days {} Hours {} Minutes",
        formatted_uptime.0, formatted_uptime.1, formatted_uptime.2
    );

    Ok(())
}

fn print_loadavg() -> Result<(), procfs::ProcError> {
    let loadavg = procfs::LoadAverage::current()?;

    println!("Load Average");
    println!("{:>3} min : {}", 1, loadavg.one);
    println!("{:>3} min : {}", 5, loadavg.five);
    println!("{:>3} min : {}", 15, loadavg.fifteen);

    Ok(())
}
