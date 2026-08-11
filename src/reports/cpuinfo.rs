use procfs::Current;
pub fn get_info() -> Result<(), procfs::ProcError> {
    let cpu = procfs::CpuInfo::current()?;
    let logical_processors = cpu.num_cores();
    let model_name = cpu.model_name(0).unwrap_or("Unknown");

    println!("CPU Model : {}", model_name);
    println!("Logical CPUs : {}", logical_processors);

    Ok(())
}
