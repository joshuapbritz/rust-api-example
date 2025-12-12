use crate::utils::database;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn sync() -> Result<(), std::io::Error> {
    let mut connection = database::get().expect("Failed to get db connection for migrations");

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    log::info!("Successfully ran diesel migrations");

    Ok(())
}
