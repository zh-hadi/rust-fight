use dotenvy::dotenv;
use std::env;
use sqlx::any::AnyConnectOptions;
use sqlx::AnyConnection;
use std::str::FromStr;



pub async fn initialization() 
{
    db_connection().await;
}
// database connection 
pub async fn db_connection()
{
    dotenv().ok();

    let db = env::var("DATABASE_URL").unwrap();

    let options = AnyConnectOptions::from_str(&db)
        .expect("Invalid database URL");

    let _conn = AnyConnection::connect_with(&options)
        .await
        .expect("failed to connect to database");

    println!("connect successfully!");


}
// migrate 
// insert data
// get data 
// update data 
// delete data 