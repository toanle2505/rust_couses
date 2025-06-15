use tokio::time::{sleep, Duration};
use tokio::task;
use reqwest::{Client, Error };
#[tokio::main]
async fn main() {
    let users = vec!["github", "microsoft", "rust-lang"];
    
    let tasks :Vec<_> = users.iter().map(|username| {
        task::spawn(async {
            fetch_github_data(username).await
        })
    }).collect();

    for task in tasks {
        match task.await {
            Ok(Ok(data)) => println!("Fetched data: {}", data),
            Ok(Err(e)) => eprintln!("Error fetching data: {}", e),
            Err(e) => eprintln!("Task failed: {}", e),
        }
    }
}

async fn fetch_data() -> String {
    // Simulate a network request with a delay
    sleep(Duration::from_secs(2)).await;
    "Data fetched".to_string()
}

async fn fetch_github_data(username : &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = format!("https://api.github.com/users/{}", username);
    
    let response = client.get(&url)
        .header("User-Agent", "reqwest")
        .send()
        .await?;
    let data = response.text().await?;
    
    Ok(data)
}
