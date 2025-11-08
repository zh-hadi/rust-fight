use tokio::time::sleep;
use tokio::time::Duration;
use dotenvy::dotenv;
use std::env;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::Error;

pub async fn initialization()-> Result<(), Error>
{

    // println!("hello world");
    // println!("sleep 2 second");

    // sleep(Duration::from_secs(2)).await;
    // println!("after back 2 second");


    dotenv().ok();

    // println!("{}", env::var("hadi").unwrap());

    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("database connection successfully!");

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(100)
        )
    "#)
    .execute(&pool)
    .await?;

    // insert data
    sqlx::query(r#"
        INSERT INTO users (name) values (?)
    "#)
        .bind("rony")
        .execute(&pool)
        .await?;
    println!("insert data done!");


    // fetch data 
    let rows = sqlx::query!("SELECT id, name FROM users").fetch_all(&pool).await?;


    for row in rows {
        println!("id: {} name: {}", row.id, row.name.unwrap());
    }

    Ok(())
}

