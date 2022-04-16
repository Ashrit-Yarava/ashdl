pub fn error(message: &str, error_code: i32) {
    eprintln!("{}", message);
    std::process::exit(error_code);
}

