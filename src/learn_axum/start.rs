use tokio::time::Duration;
use tokio::time::sleep;

pub async fn initialization()
{
    println!("hiiiii");
    sleep(Duration::from_secs(2)).await;

    println!("hello world");
}