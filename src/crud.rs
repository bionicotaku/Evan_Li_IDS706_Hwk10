use rusqlite::{params, Connection, OptionalExtension, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_name: &str) -> Result<Self> {
        let conn = Connection::open(db_name)?;
        println!("Connected to database: {}", db_name);
        Ok(Database { conn })
    }

    // User CRUD operations
    pub fn create_user(
        &self,
        email: &str,
        firstname: &str,
        lastname: &str,
        address: &str,
        balance: f64,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO users (email, firstname, lastname, address, balance)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![email, firstname, lastname, address, balance],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_user(
        &self,
        user_id: i64,
    ) -> Result<Option<(i64, String, String, String, String, f64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT user_id, email, firstname, lastname, address, balance 
             FROM users WHERE user_id = ?1",
        )?;

        stmt.query_row(params![user_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ))
        })
        .optional()
    }

    pub fn update_user(
        &self,
        user_id: i64,
        email: Option<&str>,
        firstname: Option<&str>,
        lastname: Option<&str>,
        address: Option<&str>,
        balance: Option<f64>,
    ) -> Result<bool> {
        let mut updates = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(email) = email {
            updates.push("email = ?");
            values.push(Box::new(email.to_string()));
        }
        if let Some(firstname) = firstname {
            updates.push("firstname = ?");
            values.push(Box::new(firstname.to_string()));
        }
        if let Some(lastname) = lastname {
            updates.push("lastname = ?");
            values.push(Box::new(lastname.to_string()));
        }
        if let Some(address) = address {
            updates.push("address = ?");
            values.push(Box::new(address.to_string()));
        }
        if let Some(balance) = balance {
            updates.push("balance = ?");
            values.push(Box::new(balance));
        }

        if updates.is_empty() {
            return Ok(false);
        }

        values.push(Box::new(user_id));
        let sql = format!("UPDATE users SET {} WHERE user_id = ?", updates.join(", "));

        let updated = self
            .conn
            .execute(&sql, rusqlite::params_from_iter(values))?;
        Ok(updated > 0)
    }

    pub fn delete_user(&self, user_id: i64) -> Result<bool> {
        let deleted = self
            .conn
            .execute("DELETE FROM users WHERE user_id = ?1", params![user_id])?;
        Ok(deleted > 0)
    }

    // Product CRUD operations
    pub fn create_product(
        &self,
        product_name: &str,
        category: &str,
        price: f64,
        stock: i32,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO products (product_name, category, price, stock)
             VALUES (?1, ?2, ?3, ?4)",
            params![product_name, category, price, stock],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_product(&self, product_id: i64) -> Result<Option<(i64, String, String, f64, i32)>> {
        let mut stmt = self.conn.prepare(
            "SELECT product_id, product_name, category, price, stock 
             FROM products WHERE product_id = ?1",
        )?;

        stmt.query_row(params![product_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            ))
        })
        .optional()
    }

    pub fn update_product(
        &self,
        product_id: i64,
        product_name: Option<&str>,
        category: Option<&str>,
        price: Option<f64>,
        stock: Option<i32>,
    ) -> Result<bool> {
        let mut updates = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(product_name) = product_name {
            updates.push("product_name = ?");
            values.push(Box::new(product_name.to_string()));
        }
        if let Some(category) = category {
            updates.push("category = ?");
            values.push(Box::new(category.to_string()));
        }
        if let Some(price) = price {
            updates.push("price = ?");
            values.push(Box::new(price));
        }
        if let Some(stock) = stock {
            updates.push("stock = ?");
            values.push(Box::new(stock));
        }

        if updates.is_empty() {
            return Ok(false);
        }

        values.push(Box::new(product_id));
        let sql = format!(
            "UPDATE products SET {} WHERE product_id = ?",
            updates.join(", ")
        );

        let updated = self
            .conn
            .execute(&sql, rusqlite::params_from_iter(values))?;
        Ok(updated > 0)
    }

    pub fn delete_product(&self, product_id: i64) -> Result<bool> {
        let deleted = self.conn.execute(
            "DELETE FROM products WHERE product_id = ?1",
            params![product_id],
        )?;
        Ok(deleted > 0)
    }
}
