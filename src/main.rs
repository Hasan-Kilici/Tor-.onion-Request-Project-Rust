use reqwest::Client;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

const MAX_RETRIES: usize = 3; // Maximum number of retry attempts
const RETRY_DELAY: Duration = Duration::from_secs(5); // Delay between retry attempts

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configure the proxy for Tor usage
    let proxy = reqwest::Proxy::all("socks5://127.0.0.1:9050").map_err(|e| {
        println!("Proxy configuration error: {:?}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    // Create the HTTP client and set a 60-second timeout
    let client = Client::builder()
        .proxy(proxy)
        .timeout(Duration::from_secs(60)) // Timeout set to 60 seconds
        .build()
        .map_err(|e| {
            println!("Error creating HTTP client: {:?}", e);
            Box::new(e) as Box<dyn Error>
        })?;

    // .onion URL
    let url = "http://check.torproject.org/";
    println!("Attempting to connect to URL: {}", url);

    let mut attempts = 0;

    // Retry logic: Attempt a maximum of 3 requests, retrying if errors occur
    while attempts < MAX_RETRIES {
        attempts += 1;
        println!("Attempt {}...", attempts);

        match send_get_request(&client, url).await {
            Ok(body) => {
                println!("Successfully received response: {}", body);
                break;
            }
            Err(e) => {
                println!(
                    "Error occurred during request: {:?}. Retrying in {} seconds.",
                    e,
                    RETRY_DELAY.as_secs()
                );
                sleep(RETRY_DELAY).await; // Wait before retrying
            }
        }

        if attempts == MAX_RETRIES {
            println!("Maximum retry attempts reached. Request failed.");
        }
    }

    Ok(())
}

async fn send_get_request(client: &Client, url: &str) -> Result<String, Box<dyn Error>> {
    // Send the GET request
    let response = client.get(url).send().await.map_err(|e| {
        println!("Error sending GET request: {:?}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    // Retrieve the response text
    let body = response.text().await.map_err(|e| {
        println!("Error reading response text: {:?}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    Ok(body)
}
