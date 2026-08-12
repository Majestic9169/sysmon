pub fn print_break() {
    println!("--------------------------------------------------------------");
}
pub fn print_num_header(num: i32, header: &str) {
    println!("[{}] {}", num, header);
    print_break();
}
pub fn print_header(header: &str) {
    println!("{}", header);
    print_break();
}
