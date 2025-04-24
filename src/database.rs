use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::time::Duration;

pub struct Database {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
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

    pub fn get_connection(
        &self
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, diesel::r2d2::PoolError> {
        self.pool.get()
    }

    // Helper function to handle database operations
    pub async fn transaction<F, T>(
        &self,
        operation: F,
    ) -> Result<T, String>
    where
        F: FnOnce(&mut PgConnection) -> Result<T, String> + Send + 'static,
        T: Send + 'static,
    {
        let pool = self.pool.clone(); // Clone the pool to move it into async block
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().map_err(|_| "DB_CONNECTION_FAILED".to_string())?;
            operation(&mut conn)
        })
        .await
        .map_err(|_| "TASK_EXECUTION_FAILED".to_string())?
    }
}
