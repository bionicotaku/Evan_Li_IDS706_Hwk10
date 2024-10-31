//! Database setup and initialization utilities

mod setup;
pub use setup::setup_database;
pub const DB_FILE: &str = "./data/mydatabase.db";

mod crud;
pub use crud::Database;
