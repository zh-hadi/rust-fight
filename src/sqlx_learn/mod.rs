use dotenvy::dotenv;
use std::env;

pub async fn initialization() 
{
    db_connection().await;
}
// database connection 
pub async fn db_connection()
{
    dotenv().ok();

    println!("{}", env::var("DATABASE_URL").expect("panci"));
}
// migrate 
// insert data
// get data 
// update data 
// delete data 