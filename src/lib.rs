use sqlx::{Connection, PgConnection, Executor};
use std::{
    env, fs,
    io::{self, Write},
    process,
};

struct Config {
    command: Command,
    migration_dir: String,
    full_path: String,
}

struct DbDetails {
    url: String,
}

enum Command {
    Up,
    Down,
}

enum MigrationFile {
    Up(String),
    Down(String),
}

impl Config {
    fn new(args: Vec<String>) -> Config {
        if args.len() != 3 || (args[1] != "down" && args[1] != "up") {
            eprintln!(
                "\
                Usage: yadbmt [command] [migration_dir]
                [command]: up/down
                "
            );
            process::exit(1);
        }
        let command = if args[1] == "up" {
            Command::Up
        } else {
            Command::Down
        };
        let migration_dir = &args[2];
        let migration_file = match command {
            Command::Up => MigrationFile::Up("up.sql".to_string()),
            Command::Down => MigrationFile::Down("down.sql".to_string()),
        };
        let full_path = match migration_file {
            MigrationFile::Up(ref val) => format!("{migration_dir}/{val}"),
            MigrationFile::Down(ref val) => format!("{migration_dir}/{val}"),
        };
        Config {
            migration_dir: migration_dir.to_string(),
            command,
            full_path,
        }
    }
}

pub async fn run(args: Vec<String>) {
    let config = Config::new(args);
    let contents = fs::read_to_string(config.full_path);
    let contents = match contents {
        Ok(val) => val,
        Err(e) => {
            let raw_code = e.raw_os_error();
            let raw_code = match raw_code {
                Some(val) => val,
                None => {
                    panic!("{e}");
                }
            };
            if raw_code == 2 {
                eprintln!(
                    "{}: migration directory not found or it is not in the right format",
                    config.migration_dir
                );
                process::exit(1);
            } else {
                panic!("{e}");
            }
        }
    };
    let db_url = env::var("DB_URL").unwrap_or_else(|_| {
        eprintln!("No DB_URL given");
        process::exit(1);
    });
    let db_details = DbDetails {
        url: db_url,
    };
    let is_dropping = match config.command {
        Command::Down => true,
        Command::Up => false,
    };
    if is_dropping {
        print!("Warning: executing the drop command can potentially destroy database info, do you want to continue? (y/n): ");
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Cannot read line");
        let answer = answer.trim();
        if answer != "y" {
            if answer == "n" {
                process::exit(1);
            }
            if answer.is_empty() {
                eprintln!("Aborting due to empty answer");
                process::exit(1);
            }
            eprintln!("Aborting due to unrecognized answer");
            process::exit(1);
        }
    }
    // export USERS_POSTGRES_URL=postgresql://$USERS_POSTGRES_USER:$encoded_pass@$USERS_POSTGRES_HOST:$USERS_POSTGRES_PORT/$USERS_POSTGRES_DATABASE?sslmode=disable
    let mut conn = PgConnection::connect(&db_details.url).await.unwrap_or_else(|e| {
        eprintln!("Error connecting to the database: {e}");
        process::exit(1);
    });
    let contents = &contents[..];
    let _ = match conn.execute(contents).await {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };
}
