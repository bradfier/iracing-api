use std::io;
use std::io::{BufRead, Write};

/// Simple CLI helper for generating your API authentication hash - useful
/// with [`iracing_api::auth::AuthRequest::new_from_hash`]
fn main() {
    let mut email = String::new();
    let stdin = io::stdin();
    print!("Email: ");
    io::stdout().flush().ok().unwrap();
    stdin.lock().read_line(&mut email).unwrap();

    let password = rpassword::prompt_password("iRacing Password: ").unwrap();
    let hash = iracing_api::auth::generate_hash(email.trim(), &password);

    println!("{}", hash);
}
