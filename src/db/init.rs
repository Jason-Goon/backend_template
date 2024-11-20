use sqlx::{SqlitePool, migrate::Migrator};

static MIGRATOR: Migrator = sqlx::migrate!(); 

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("Running migrations...");

    let result = MIGRATOR.run(pool).await;

    match result {
        Ok(_) => println!("Migrations applied successfully."),
        Err(e) => println!("Error applying migrations: {:?}", e),
    }

    Ok(())
}
