use procfs::Current;

pub fn print_mem_info() -> Result<(), procfs::ProcError> {
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
