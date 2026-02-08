use sqlx::PgPool;
use std::net::TcpListener;

use newsletter::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create Postgres connection pool
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    println!("{db_pool:?}");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("http://127.0.0.1:{}", port);
    run(listener)?.await
}
