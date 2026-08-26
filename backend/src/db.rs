use sqlx::postgres::{ PgPool, PgPoolOptions };
// creates a connection pool to our Supabase Postgres database.

pub async fn create_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid");

    PgPoolOptions::new()
        // max_connections caps how many simultaneous DB connections we open
        .max_connections(5)
        .connect(&database_url).await
        .expect("Failed to connect to Postgres")
}
