//! Database connection pool abstraction using Diesel and r2d2.
//!
//! This module provides a convenient wrapper around a PostgreSQL connection pool for use in async applications.
//!
//! # Example
//!
//! ```rust
//! let db = Database::new("postgres://user:password@localhost/dbname").unwrap();
//! let conn = db.get_connection().unwrap();
//! // Use conn for queries...
//! ```
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
// use diesel::Connection;
use std::time::Duration;

/// Abstraction over a Diesel PostgreSQL connection pool.
pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    /// Create a new database connection pool from the given URL.
    ///
    /// # Arguments
    /// * `database_url` - PostgreSQL connection string
    pub fn new(database_url: &str) -> Result<Self, diesel::r2d2::PoolError> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .connection_timeout(Duration::from_secs(60))
            .max_size(32) // Adjust based on your application's needs
            .min_idle(Some(8)) // Keep minimum idle connections
            .idle_timeout(Some(Duration::from_secs(600))) // 10 minutes idle timeout
            .max_lifetime(Some(Duration::from_secs(3600))) // 1 hour max lifetime
            .test_on_check_out(true) // Verify connection on checkout
            .build(manager)?;
        Ok(Self { pool })
    }

    /// Get a pooled database connection.
    pub fn get_connection(
        &self
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::r2d2::PoolError> {
        self.pool.get()
    }

    // Keep your original transaction method but with proper diesel transaction handling
    /// Execute a write operation (insert, update, delete) in a transaction.
    /// Automatically rolls back on error.
    pub async fn transaction<F, T>(
        &self,
        operation: F,
    ) -> Result<T>
    where
        F: FnOnce(&mut PgConnection) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;

            // Use diesel's transaction method for proper rollback handling
            // conn.transaction(|conn| {
            //     operation(conn).map_err(|_e| diesel::result::Error::RollbackTransaction)
            // })
            // .map_err(|e| match e {
            //     diesel::result::Error::RollbackTransaction => "TRANSACTION_ROLLED_BACK".to_string(),
            //     _ => format!("TRANSACTION_FAILED:{}", e),
            // })
            operation(&mut conn) // Auto-deref handles the conversion
        })
        .await?
    }

    /// Execute a read-only operation using a pooled connection.
    /// Recommended for SELECT/GET queries.
    pub async fn execute<F, T>(
        &self,
        operation: F,
    ) -> Result<T>
    where
        F: FnOnce(&mut PgConnection) -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get()?;
            operation(&mut conn) // Auto-deref handles the conversion
        })
        .await?
    }

    /// Run a simple health check query to verify database connectivity.
    pub async fn health_check(&self) -> Result<()> {
        use diesel::sql_query;
        use diesel::RunQueryDsl;

        self.execute(|conn| {
            sql_query("SELECT 1").execute(conn)?;
            Ok(())
        })
        .await
    }

    /// Get statistics about the current state of the connection pool.
    /// Returns (idle_connections, used_connections).
    pub fn pool_stats(&self) -> (u32, u32) {
        let state = self.pool.state();
        (state.connections, state.idle_connections)
    }
}

// Usage examples
// #[cfg(test)]
// mod usage_example {
//     use super::*;
//     use diesel::sql_query;
//     use diesel::RunQueryDsl;
//     use serde_json::Value;
//
//     #[derive(diesel::QueryableByName)]
//     struct SQLJsonResult {
//         #[diesel(sql_type = diesel::sql_types::Json)]
//         data: Value,
//     }
//
//     async fn example_usage(
//         database_url: &str,
//         user_id: i64,
//     ) -> Result<Value, String> {
//         let db = Database::new(database_url).map_err(|e| e.to_string())?;
//
//         // Your exact usage pattern - this will work exactly as before
//         db.transaction(move |conn| {
//             let result: SQLJsonResult =
//                 sql_query("SELECT row_to_json(t.*) as data FROM get_dashboard_overview($1) t")
//                     .bind::<diesel::sql_types::Bigint, _>(user_id)
//                     .get_result(conn)
//                     .map_err(|e| format!("DASHBOARD_QUERY_FAILED:{}", e))?;
//
//             Ok(result.data)
//         })
//         .await
//     }
//
//     async fn example_read_only(
//         database_url: &str,
//         user_id: i64,
//     ) -> Result<Value, String> {
//         let db = Database::new(database_url).map_err(|e| e.to_string())?;
//
//         // For read-only operations, use execute (no transaction overhead)
//         db.execute(move |conn| {
//             let result: SQLJsonResult =
//                 sql_query("SELECT row_to_json(t.*) as data FROM get_user_profile($1) t")
//                     .bind::<diesel::sql_types::Bigint, _>(user_id)
//                     .get_result(conn)
//                     .map_err(|e| format!("USER_PROFILE_FAILED:{}", e))?;
//
//             Ok(result.data)
//         })
//         .await
//     }
// }
