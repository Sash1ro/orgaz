fn main() {
    if let Err(err) = orgaz::run() {
        orgaz::cli::report_error(&err);
        std::process::exit(1);
    }
}
