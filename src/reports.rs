use crate::utils::print_header;
use procfs::Current;

pub fn print_report() -> Result<(), procfs::ProcError> {
    print_header(1, "CPU & MEMORY INFORMATION");
    print_cpu_info()?;
    print_mem_info()?;

    Ok(())
}

fn print_cpu_info() -> Result<(), procfs::ProcError> {
    let cpu = procfs::CpuInfo::current()?;
    let logical_processors = cpu.num_cores();
    let model_name = cpu.model_name(0).unwrap_or("Unknown");

    println!("CPU Model : {}", model_name);
    println!("Logical CPUs : {}", logical_processors);

    Ok(())
}

fn print_mem_info() -> Result<(), procfs::ProcError> {
    let mem = procfs::Meminfo::current()?;

    println!(
        "Total Memory : {}",
        human_memsize::human_size(mem.mem_total)
    );
    println!(
        "Available Memory : {}",
        human_memsize::human_size(mem.mem_available.unwrap_or(0))
    );
    println!("Free Memory : {}", human_memsize::human_size(mem.mem_free));

    println!(
        "Memory Usage : {:.1}%",
        100f64 * (1f64 - (mem.mem_available.unwrap_or(0) as f64 / mem.mem_total as f64))
    );

    Ok(())
}
