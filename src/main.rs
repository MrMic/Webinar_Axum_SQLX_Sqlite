//                   ╔═══════════════════════════════════════╗
//                   ║ YOUTUBE - AXUM / CRUD / SQLX / SQLITE ║
//                   ╚═══════════════════════════════════════╝

mod db;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env if available
    dotenv::dotenv().ok();

    // Initialize the database and obtain the connection pool
    let connection_pool = init_db().await?;

    Ok(())
}
