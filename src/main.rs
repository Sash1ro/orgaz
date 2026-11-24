fn main() {
    if let Err(err) = orgaz::run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
