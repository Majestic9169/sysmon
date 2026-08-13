use procfs::Current;

pub fn print_mem_info() -> Result<String, procfs::ProcError> {
    let mut result: String = String::new();

    let mem = procfs::Meminfo::current()?;

    let avail_mem = mem
        .mem_available
        .ok_or(procfs::ProcError::Other(String::from(
            "Missing Available Memory",
        )))?;

    result += &format!(
        "Total Memory : {}\n",
        human_memsize::human_size(mem.mem_total)
    );
    result += &format!(
        "Available Memory : {}\n",
        human_memsize::human_size(avail_mem)
    );
    result += &format!(
        "Free Memory : {}\n",
        human_memsize::human_size(mem.mem_free)
    );

    result += &format!(
        "Memory Usage : {:.1}%\n",
        100f64 * (1f64 - (avail_mem as f64 / mem.mem_total as f64))
    );

    Ok(result)
}
