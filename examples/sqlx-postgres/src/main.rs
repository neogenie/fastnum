use sqlx::postgres::PgPoolOptions;
use fastnum::{dec128, udec128, D128, UD128};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:example@localhost").await?;

    let row: (UD128,D128) = sqlx::query_as("SELECT $1, $2")
        .bind(udec128!(3.14159265358979323846264))
        .bind(dec128!(-3.14159265358979323846264))
        .fetch_one(&pool).await?;

    assert_eq!(row.0, udec128!(3.14159265358979323846264));
    assert_eq!(row.1, dec128!(-3.14159265358979323846264));

    Ok(())
}