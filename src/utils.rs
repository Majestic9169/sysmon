pub fn print_break() -> String {
    format!("--------------------------------------------------------------\n")
}
pub fn print_num_header(num: i32, header: &str) -> String {
    format!("[{}] {}\n", num, header) + &print_break()
}
pub fn print_header(header: &str) -> String {
    format!("{}\n", header) + &print_break()
}

pub fn clear_screen() {
    // Source - https://stackoverflow.com/a/66911945
    // Posted by Dan Charousek
    // Retrieved 2026-08-12, License - CC BY-SA 4.0

    print!("{esc}c", esc = 27 as char);
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
