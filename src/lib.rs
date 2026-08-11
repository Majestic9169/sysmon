mod reports;

fn print_header(num: i32, header: &str) {
    println!("[{}] {}", num, header);
    println!("--------------------------------------------------------------");
}
pub fn cpu_and_mem_info() {
    print_header(1, "CPU & MEMORY INFORMATION");
    let _ = reports::cpuinfo::get_info();
    let _ = reports::meminfo::get_info();
}
