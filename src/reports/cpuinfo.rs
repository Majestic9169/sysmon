use procfs::Current;
pub fn print_cpu_info() -> Result<String, procfs::ProcError> {
    let mut result: String = String::new();

    let cpu = procfs::CpuInfo::current()?;
    let logical_processors = cpu.num_cores();
    let model_name = cpu
        .model_name(0)
        .ok_or(procfs::ProcError::Other(String::from(
            "Missing CPU model name",
        )))?;

    result += &format!("CPU Model : {}\n", model_name);
    result += &format!("Logical CPUs : {}\n", logical_processors);

    Ok(result)
}
