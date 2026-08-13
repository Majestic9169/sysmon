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

pub fn print_report() -> Result<String, procfs::ProcError> {
    let mut report: String = String::new();

    report += &format!("==============================================================\n");
    report += &format!("              PROC MONITOR - SYSTEM INFORMATION               \n");
    report += &format!("==============================================================\n");

    report += &utils::print_num_header(1, "CPU & MEMORY INFORMATION");
    report += &reports::cpuinfo::print_cpu_info()?;
    report += &reports::meminfo::print_mem_info()?;
    report += &utils::print_break();

    report += &utils::print_num_header(2, "CPU UTILIZATION & SYSTEM STATISTICS");
    report += &reports::utilization::print_cpu_utilization()?;
    report += &reports::stats::print_uptime()?;
    report += &reports::stats::print_loadavg()?;
    report += &utils::print_break();

    report += &utils::print_num_header(3, "RUNNNING PROCESSES");
    report += &reports::processes::print_all_processes()?;
    report += &utils::print_break();

    report += &utils::print_num_header(4, "TOP 5 MEMORY CONSUMERS");
    report += &reports::processes::print_top_mem_processes()?;
    report += &utils::print_break();
    report += &utils::print_header("TOP 5 CPU CONSUMERS");
    report += &reports::processes::print_top_cpu_processes()?;
    report += &utils::print_break();

    Ok(report)
}

fn main() {
    let args = Cli::parse();

    if let Some(mut interval) = args.time_period {
        // NOTE: account for 2 second sampling delay
        if interval >= 2 {
            interval = interval - 2;
        }

        loop {
            match print_report() {
                Ok(report) => println!("{}", report),
                Err(err) => {
                    eprintln!("Something went wrong");
                    eprintln!("{}", err);
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(interval));
            clear_screen();
        }
    } else {
        match print_report() {
            Ok(report) => println!("{}", report),
            Err(err) => {
                eprintln!("Something went wrong");
                eprintln!("{}", err);
            }
        }
    }
}
