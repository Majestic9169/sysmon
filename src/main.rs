mod reports;
mod utils;

fn main() {
    if let Err(err) = reports::print_report() {
        eprintln!("Something went wrong");
        eprintln!("{}", err);
    };
}
