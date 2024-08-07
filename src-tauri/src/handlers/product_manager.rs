use crate::{
    database::Database,
    models::user::User,
    repository::{
        category_repository::CategoryRepository, product_repository::ProductRepository,
        supplier_repository::SupplierRepository, user_repository::UserRepository, Repository,
    },
};
use sqlx::FromRow;
use tauri::State;

use crate::models::category::Category;
use crate::models::product::Product;
use crate::models::supplier::Supplier;

// Define el tipo Record si aún no está definido
#[derive(Debug, serde::Serialize, FromRow)]
pub struct RecordProducByCategory {
    id: i64,
    category_name: String,
    product_count: i64,
}

/* products */

#[tauri::command]
pub async fn create_product(
    state: State<'_, Database>,
    name: &str,
    description: Option<String>,
    category_id: i64,
    supplier_id: i64,
) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let new_product = Product {
        id: 0,
        name: name.to_string(),
        description,
        category_id,
        supplier_id,
        price_purchase: 0.0,
        price_sell: 0.0,
        stock_initial: 0,
        stock_current: 0,
        utility: 0.0,
    };

    let mut product_rep = ProductRepository::new(pool_conn);

    if let Err(e) = product_rep.create(new_product).await {
        return Err(format!("Error al crear producto: {}", e,));
    }

    Ok(())
}

#[tauri::command]
pub async fn get_products(state: State<'_, Database>) -> Result<Vec<Product>, String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut product_rep = ProductRepository::new(pool_conn);
    let products = product_rep
        .find_all()
        .await
        .map_err(|e| format!("Error en la obtención de productos: {}", e))?;

    Ok(products)
}

#[tauri::command]
pub async fn delete_product(state: State<'_, Database>, product_id: i64) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut product_rep = ProductRepository::new(pool_conn);

    product_rep
        .delete(product_id)
        .await
        .map_err(|e| format!("Error en la eliminación del producto: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_inventory(
    state: State<'_, Database>,
    product_id: i64,
    price_purchase: f64,
    price_sell: f64,
) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut product_rep = ProductRepository::new(pool_conn);

    let product_old = match product_rep.find_by_id(product_id).await {
        Ok(product) => product,
        Err(_e) => None,
    };

    let product_old = product_old.expect("Error al obtener el producto actual.");
    let product_updated = Product {
        id: product_id,
        price_purchase,
        price_sell,
        ..product_old
    };

    product_rep
        .update(product_updated)
        .await
        .map_err(|e| format!("Error en la actualización del producto: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_utility_product(
    state: State<'_, Database>,
    product_id: i64,
    stock_current: i64,
    utility: f64,
) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut product_rep = ProductRepository::new(pool_conn);

    let product_old = match product_rep.find_by_id(product_id).await {
        Ok(product) => product,
        Err(_e) => None,
    };

    let product_old = product_old.expect("Error al obtener el producto actual.");
    let product_updated = Product {
        id: product_id,
        stock_current,
        utility,
        ..product_old
    };

    product_rep
        .update(product_updated)
        .await
        .map_err(|e| format!("Error en la actualización del producto: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_product(
    state: State<'_, Database>,
    product_id: i64,
    name: &str,
    description: Option<String>,
    category_id: i64,
    supplier_id: i64,
) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut product_rep = ProductRepository::new(pool_conn);

    let product_old = match product_rep.find_by_id(product_id).await {
        Ok(product) => product,
        Err(_e) => None,
    };

    let product_old = product_old.expect("Error al obtener el producto actual.");
    let product_updated = Product {
        id: product_id,
        name: name.to_string(),
        description,
        category_id,
        supplier_id,
        ..product_old
    };

    product_rep
        .update(product_updated)
        .await
        .map_err(|e| format!("Error en la actualización del producto: {}", e))?;

    Ok(())
}

/* categories */

#[tauri::command]
pub async fn create_category(state: State<'_, Database>, name: &str) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let new_category = Category {
        id: 0,
        name: name.to_string(),
    };

    let mut category_rep = CategoryRepository::new(pool_conn);

    if let Err(e) = category_rep.create(new_category).await {
        return Err(format!("Error al crear categoria: {}", e,));
    }

    Ok(())
}

#[tauri::command]
pub async fn get_categories(state: State<'_, Database>) -> Result<Vec<Category>, String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut category_rep = CategoryRepository::new(pool_conn);
    let categories = category_rep
        .find_all()
        .await
        .map_err(|e| format!("Error en la obtención de categorias: {}", e))?;

    Ok(categories)
}

#[tauri::command]
pub async fn delete_category(state: State<'_, Database>, category_id: i64) -> Result<(), String> {
    let mut pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let query = sqlx::query!(
        "UPDATE product SET category_id = 1 WHERE category_id = ?",
        category_id
    );

    query.execute(&mut *pool_conn).await.map_err(|e| {
        format!(
            "Error al actualizar la categoría asociada a los productos: {}",
            e
        )
    })?;

    let mut category_rep = CategoryRepository::new(pool_conn);
    category_rep
        .delete(category_id)
        .await
        .map_err(|e| format!("Error en la eliminación de la categoria: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_category(
    state: State<'_, Database>,
    category_id: i64,
    name: &str,
) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut category_rep = CategoryRepository::new(pool_conn);
    let category_update = Category {
        id: category_id,
        name: name.to_string(),
    };

    category_rep
        .update(category_update)
        .await
        .map_err(|e| format!("Error en la actualización de la categoria: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_amount_products_by_category(
    state: State<'_, Database>,
) -> Result<Vec<RecordProducByCategory>, String> {
    let mut pool_conn = match state.clone().get_connection().await {
        Ok(conn) => conn,
        Err(e) => return Err(format!("Failed to get database connection: {}", e)),
    };

    let query = sqlx::query_as!(
        RecordProducByCategory,
        "SELECT c.id, c.name AS category_name, COUNT(p.id) AS product_count 
        FROM category c 
        LEFT JOIN product p ON c.id = p.category_id 
        GROUP BY c.name"
    );

    match query.fetch_all(&mut *pool_conn).await {
        Ok(results) => Ok(results),
        Err(e) => Err(format!("Failed to execute query: {}", e)),
    }
}

/* suppliers */

#[tauri::command]
pub async fn create_supplier(
    state: State<'_, Database>,
    name: &str,
    email: &str,
    phone: &str,
    address: &str,
) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let new_supplier = Supplier {
        id: 0,
        name: name.to_string(),
        email: email.to_string(),
        phone: phone.to_string(),
        address: address.to_string(),
    };

    let mut supplier_rep = SupplierRepository::new(pool_conn);

    if let Err(e) = supplier_rep.create(new_supplier).await {
        return Err(format!("Error al crear proveedor: {}", e,));
    }

    Ok(())
}

#[tauri::command]
pub async fn get_suppliers(state: State<'_, Database>) -> Result<Vec<Supplier>, String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut supplier_rep = SupplierRepository::new(pool_conn);
    let suppliers = supplier_rep
        .find_all()
        .await
        .map_err(|e| format!("Error en la obtención de proveedores: {}", e))?;

    Ok(suppliers)
}

#[tauri::command]
pub async fn delete_supplier(state: State<'_, Database>, supplier_id: i64) -> Result<(), String> {
    let mut pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let query = sqlx::query!(
        "UPDATE product SET supplier_id = 1 WHERE supplier_id = ?",
        supplier_id
    );

    query.execute(&mut *pool_conn).await.map_err(|e| {
        format!(
            "Error al actualizar el proveedor asociado a los productos: {}",
            e
        )
    })?;

    let mut supplier_rep = SupplierRepository::new(pool_conn);
    supplier_rep
        .delete(supplier_id)
        .await
        .map_err(|e| format!("Error en la eliminación del proveedor: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_supplier(
    state: State<'_, Database>,
    supplier_id: i64,
    name: &str,
    email: &str,
    phone: &str,
    address: &str,
) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut supplier_rep = SupplierRepository::new(pool_conn);
    let supplier_update = Supplier {
        id: supplier_id,
        name: name.to_string(),
        email: email.to_string(),
        phone: phone.to_string(),
        address: address.to_string(),
    };

    supplier_rep
        .update(supplier_update)
        .await
        .map_err(|e| format!("Error en la actualización del proveedor: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn create_user(
    state: State<'_, Database>,
    email: &str,
    password: &str,
    role_id: i64,
) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let new_user = User {
        id: 0,
        email: email.to_string(),
        password: password.to_string(),
    };

    let mut user_rep = UserRepository::new(pool_conn);

    if let Err(e) = user_rep.create(new_user).await {
        return Err(format!("Error al crear usuario: {}", e,));
    }

    Ok(())
}

#[tauri::command]
pub async fn get_users(state: State<'_, Database>) -> Result<Vec<User>, String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut user_rep = UserRepository::new(pool_conn);
    let users = user_rep
        .find_all()
        .await
        .map_err(|e| format!("Error en la obtención de usuarios: {}", e))?;

    Ok(users)
}

#[tauri::command]
pub async fn delete_user(state: State<'_, Database>, user_id: i64) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut user_rep = UserRepository::new(pool_conn);
    user_rep
        .delete(user_id)
        .await
        .map_err(|e| format!("Error en la eliminación del usuario: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn update_user(
    state: State<'_, Database>,
    user_id: i64,
    email: &str,
    password: &str,
) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut user_rep = UserRepository::new(pool_conn);
    let user_update = User {
        id: user_id,
        email: email.to_string(),
        password: password.to_string(),
    };

    user_rep
        .update(user_update)
        .await
        .map_err(|e| format!("Error en la actualización del usuario: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn add_stock(
    state: State<'_, Database>,
    product_id: i64,
    stock_initial: i64,
    stock_current: i64,
) -> Result<(), String> {
    let pool_conn = state
        .clone()
        .get_connection()
        .await
        .map_err(|_| "Failed to get database connection".to_string())?;

    let mut product_rep = ProductRepository::new(pool_conn);

    let product_old = match product_rep.find_by_id(product_id).await {
        Ok(product) => product,
        Err(_e) => None,
    };

    let product_old = product_old.expect("Error al obtener el producto actual.");
    let product_updated = Product {
        id: product_id,
        stock_initial,
        stock_current,
        ..product_old
    };

    product_rep
        .update(product_updated)
        .await
        .map_err(|e| format!("Error en agregar del stock: {}", e))?;

    Ok(())
}
