use procfs::Current;
pub fn print_cpu_info() -> Result<(), procfs::ProcError> {
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
