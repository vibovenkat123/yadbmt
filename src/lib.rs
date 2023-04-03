use std::{fs, env};
pub fn run(args: Vec<String>) {
    let file_path = &args[1];
    let db_pass = env::var("DB_PASS").unwrap_or("123".to_string());
    let db_user = env::var("DB_USER").unwrap_or("bob".to_string());
    let db_host = env::var("DB_HOST").unwrap_or("localhost".to_string());
    let db_port = env::var("DB_PORT").unwrap_or("5432".to_string());
    let db_name = env::var("DB_NAME").unwrap_or("db".to_string());
    let contents = fs::read_to_string(file_path).unwrap();
    println!("{contents}");
    println!("{db_pass}");
    println!("{db_user}");
    println!("{db_host}");
    println!("{db_port}");
    println!("{db_name}");
}
