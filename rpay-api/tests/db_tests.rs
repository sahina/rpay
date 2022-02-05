use sqlx::postgres::PgPoolOptions;

use rpay_core::vars;

#[actix_rt::test]
async fn db_connection() -> anyhow::Result<()> {
    let db_url = vars::database_url();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str()).await?;

    let row: (i64, ) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}