use super::Repository;
use super::SqlxError;
use sqlx::pool::PoolConnection;
use sqlx::Sqlite;

use crate::models::supplier::Supplier;

pub struct SupplierRepository {
    pool: PoolConnection<Sqlite>,
}

impl SupplierRepository {
    pub fn new(pool: PoolConnection<Sqlite>) -> Self {
        Self { pool }
    }
}

impl Repository<Supplier> for SupplierRepository {
    async fn find_all(&mut self) -> Result<Vec<Supplier>, SqlxError> {
        let suppliers = sqlx::query_as!(Supplier, "SELECT * FROM supplier;")
            .fetch_all(&mut *self.pool)
            .await?;

        Ok(suppliers)
    }

    async fn find_by_id(&mut self, supplier_id: i64) -> Result<Option<Supplier>, SqlxError> {
        let supplier =
            sqlx::query_as!(Supplier, "SELECT * FROM supplier WHERE id = ?", supplier_id)
                .fetch_optional(&mut *self.pool)
                .await?;
        Ok(supplier)
    }

    async fn create(&mut self, supplier: Supplier) -> Result<(), SqlxError> {
        sqlx::query!(
            "INSERT INTO supplier (name, address, phone, email) VALUES (?, ?, ?, ?)",
            supplier.name,
            supplier.address,
            supplier.phone,
            supplier.email,
        )
        .execute(&mut *self.pool)
        .await?;

        Ok(())
    }

    async fn update(&mut self, supplier: Supplier) -> Result<(), SqlxError> {
        sqlx::query!(
            "UPDATE supplier SET name = ? WHERE id = ?",
            supplier.name,
            supplier.id
        )
        .execute(&mut *self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&mut self, supplier_id: i64) -> Result<(), SqlxError> {
        sqlx::query!("DELETE FROM supplier WHERE id = ?", supplier_id)
            .execute(&mut *self.pool)
            .await?;

        Ok(())
    }
}
