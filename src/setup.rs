use crate::DB_FILE;
use rusqlite::{Connection, Result as SqliteResult};
use std::fs;
use std::path::Path;
use std::process::{exit, Command};

pub fn setup_database() -> SqliteResult<()> {
    // Step 1: Create the database file
    create_database()?;

    // Step 2: Create or update the database schema
    run_sql_create_file("./data/createTables.sql")?;

    // Step 3: Generate sample data
    run_python_script("./data/gen.py");

    // Step 4: Load data into the database
    run_sql_load_file("./data/load.sql");

    // Step 5: Delete CSV files
    delete_csv_files("./data");

    println!("\nDatabase setup and created all the example data.");
    Ok(())
}

fn create_database() -> SqliteResult<()> {
    if !Path::new(DB_FILE).exists() {
        let _conn = Connection::open(DB_FILE)?;
        println!("Created new database file: {}", DB_FILE);
    } else {
        println!("Database file {} already exists", DB_FILE);
    }
    Ok(())
}

fn run_sql_create_file(file_path: &str) -> SqliteResult<()> {
    let conn = Connection::open(DB_FILE)?;
    let sql_script = fs::read_to_string(file_path).expect(&format!("Failed to read {}", file_path));

    conn.execute_batch(&sql_script)?;
    println!("Successfully executed {}", file_path);
    Ok(())
}

fn run_sql_load_file(file_path: &str) {
    println!("Loading data from {}...", file_path);

    let output = Command::new("sqlite3")
        .arg(DB_FILE)
        .arg(format!(".read {}", file_path))
        .output()
        .expect("Failed to execute SQLite command");

    if output.status.success() {
        println!("Successfully loaded data from {}", file_path);
    } else {
        eprintln!("Error executing SQLite shell command");
        eprintln!("Error output: {}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }
}

fn run_python_script(script_path: &str) {
    println!("Running {}... This may take a while...", script_path);
    let output = Command::new("python")
        .arg(script_path)
        .output()
        .expect("Failed to execute Python script");

    if output.status.success() {
        println!("Successfully completed {}", script_path);
    } else {
        eprintln!(
            "Error running {}: {}",
            script_path,
            String::from_utf8_lossy(&output.stderr)
        );
        exit(1);
    }
}

fn delete_csv_files(directory: &str) {
    let paths = fs::read_dir(directory).expect("Failed to read directory");

    for path in paths {
        if let Ok(entry) = path {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("csv") {
                if let Err(e) = fs::remove_file(&path) {
                    eprintln!("Error deleting {:?}: {}", path, e);
                } else {
                    println!("Deleted {:?}", path);
                }
            }
        }
    }
}
