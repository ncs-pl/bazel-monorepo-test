/// Print the given message
pub fn print(message: &str) {
    println!("{}", message);
}

/// Print the given message with the given prefix in uppercase
pub fn print_with_prefix(prefix: &str, message: &str) {
    println!("{}: {}", prefix.to_uppercase(), message)
}
