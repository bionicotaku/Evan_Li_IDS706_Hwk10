use rust_proj::{setup_database, Database, DB_FILE};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = setup_database() {
        eprintln!("Error setting up database: {}", e);
        std::process::exit(1);
    }

    let db = Database::new(DB_FILE)?;

    // User CRUD examples
    let user_id = db.create_user("john@example.com", "John", "Doe", "123 Main St", 100.0)?;

    let user = db.get_user(user_id)?;
    println!(
        "Created user with ID: {}, Retrieved user: {:?}",
        user_id, user
    );

    let updated = db.update_user(
        user_id,
        Some("johndoe@example.com"),
        None,
        None,
        None,
        Some(200.0),
    )?;
    let user = db.get_user(user_id)?;
    println!(
        "Update user status: {}, Retrieved user: {:?}",
        updated, user
    );

    let deleted = db.delete_user(user_id)?;
    let user = db.get_user(user_id)?;
    println!(
        "Delete user status: {}, Retrieved user: {:?}",
        deleted, user
    );

    // Product CRUD examples
    let product_id = db.create_product("Laptop", "Electronics", 999.99, 10)?;
    let product = db.get_product(product_id)?;
    println!(
        "Created product with ID: {}, Retrieved product: {:?}",
        product_id, product
    );

    let updated = db.update_product(product_id, None, None, Some(899.99), Some(15))?;
    let product = db.get_product(product_id)?;
    println!(
        "Update product status: {}, Retrieved product: {:?}",
        updated, product
    );

    let deleted = db.delete_product(product_id)?;
    let product = db.get_product(product_id)?;
    println!(
        "Delete product status: {}, Retrieved product: {:?}",
        deleted, product
    );

    Ok(())
}
