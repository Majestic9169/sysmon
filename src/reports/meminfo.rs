use human_memsize;
use procfs::Current;

pub fn get_info() -> Result<(), procfs::ProcError> {
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
