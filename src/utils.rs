pub fn print_break() {
    println!("--------------------------------------------------------------");
}
pub fn print_header(num: i32, header: &str) {
    println!("[{}] {}", num, header);
    print_break();
}
