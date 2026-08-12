mod reports;
mod utils;

pub fn print_report() -> Result<(), procfs::ProcError> {
    println!("==============================================================");
    println!("              PROC MONITOR - SYSTEM INFORMATION               ");
    println!("==============================================================");

    utils::print_header(1, "CPU & MEMORY INFORMATION");
    reports::cpuinfo::print_cpu_info()?;
    reports::meminfo::print_mem_info()?;
    utils::print_break();

    utils::print_header(2, "CPU UTILIZATION & SYSTEM STATISTICS");
    reports::utilization::print_cpu_utilization()?;
    reports::stats::print_uptime()?;
    reports::stats::print_loadavg()?;
    utils::print_break();

    Ok(())
}

fn main() {
    if let Err(err) = print_report() {
        eprintln!("Something went wrong");
        eprintln!("{}", err);
    };
}
