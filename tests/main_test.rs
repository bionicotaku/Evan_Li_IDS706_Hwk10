use rust_proj::{Database, DB_FILE};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_crud_operations() {
        let db = Database::new(DB_FILE).expect("Failed to create database");

        // 测试创建用户
        let user_id = db
            .create_user("test@example.com", "Test", "User", "123 Test St", 150.0)
            .expect("Failed to create user");

        // 测试获取用户
        let user = db.get_user(user_id).expect("Failed to get user").unwrap();
        assert_eq!(user.1, "test@example.com");
        assert_eq!(user.5, 150.0);

        // 测试更新用户
        let updated = db
            .update_user(
                user_id,
                Some("updated@example.com"),
                None,
                None,
                None,
                Some(200.0),
            )
            .expect("Failed to update user");
        assert!(updated);

        // 测试删除用户
        let deleted = db.delete_user(user_id).expect("Failed to delete user");
        assert!(deleted);
    }

    #[test]
    fn test_product_crud_operations() {
        let db = Database::new(DB_FILE).expect("Failed to create database");

        // 测试创建产品
        let product_id = db
            .create_product("Test Product", "Test Category", 99.99, 5)
            .expect("Failed to create product");

        // 测试获取产品
        let product = db
            .get_product(product_id)
            .expect("Failed to get product")
            .unwrap();
        assert_eq!(product.1, "Test Product");
        assert_eq!(product.3, 99.99);

        // 测试更新产品
        let updated = db
            .update_product(product_id, None, None, Some(89.99), Some(10))
            .expect("Failed to update product");
        assert!(updated);

        // 测试删除产品
        let deleted = db
            .delete_product(product_id)
            .expect("Failed to delete product");
        assert!(deleted);
    }
}
