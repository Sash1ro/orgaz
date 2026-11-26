use owo_colors::OwoColorize;

pub fn error(message: impl std::fmt::Display) {
    eprintln!("{} {}", "✖".red().bold(), message);
}

pub fn warning(message: impl std::fmt::Display) {
    eprintln!("{} {}", "⚠".yellow().bold(), message);
}

pub fn success(message: impl std::fmt::Display) {
    println!("{} {}", "✔".green().bold(), message);
}

pub fn info(message: impl std::fmt::Display) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

pub fn step(message: impl std::fmt::Display) {
    println!("{} {}", "›".cyan().bold(), message);
}
