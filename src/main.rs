use clap::Parser;

use crate::utils::clear_screen;

mod reports;
mod utils;

#[derive(Parser)]
#[command(about = "System Monitor for Linux")]
#[command(next_line_help = true)]
struct Cli {
    #[arg(
        short = 'T',
        value_name = "interval",
        help = "time interval to refresh for live mode"
    )]
    time_period: Option<u64>,
}

pub fn print_report() -> Result<(), procfs::ProcError> {
    println!("==============================================================");
    println!("              PROC MONITOR - SYSTEM INFORMATION               ");
    println!("==============================================================");

    utils::print_num_header(1, "CPU & MEMORY INFORMATION");
    reports::cpuinfo::print_cpu_info()?;
    reports::meminfo::print_mem_info()?;
    utils::print_break();

    utils::print_num_header(2, "CPU UTILIZATION & SYSTEM STATISTICS");
    reports::utilization::print_cpu_utilization()?;
    reports::stats::print_uptime()?;
    reports::stats::print_loadavg()?;
    utils::print_break();

    utils::print_num_header(3, "RUNNNING PROCESSES");
    reports::processes::print_all_processes()?;
    utils::print_break();

    utils::print_num_header(4, "TOP 5 MEMORY CONSUMERS");
    reports::processes::print_top_mem_processes()?;
    utils::print_break();
    utils::print_header("TOP 5 CPU CONSUMERS");
    reports::processes::print_top_cpu_processes()?;
    utils::print_break();

    Ok(())
}

fn main() {
    let args = Cli::parse();

    if let Some(interval) = args.time_period {
        loop {
            if let Err(err) = print_report() {
                eprintln!("Something went wrong");
                eprintln!("{}", err);
            };
            std::thread::sleep(std::time::Duration::from_secs(interval));
            clear_screen();
        }
    } else {
        if let Err(err) = print_report() {
            eprintln!("Something went wrong");
            eprintln!("{}", err);
        };
    }
}
