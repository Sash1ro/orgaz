use anyhow::Error;
use owo_colors::OwoColorize;

pub fn report_error(err: &Error) {
    eprintln!("{} {}", "ERROR".red().bold(), err);

    for cause in err.chain().skip(1) {
        eprintln!("  {} {}", "Caused by:".yellow(), cause);
    }
}
