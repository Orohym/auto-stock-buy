use std::error::Error;

use thirtyfour::prelude::*;

use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize)]
struct SecretData {
    id: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct Stock {
    name: String,
    id: String,
    quantity: u32,
}

#[derive(Debug, Deserialize)]
struct StockIdData {
    data: Vec<Stock>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reading secret.json
    let mut secret_file = File::open("./secret.json")?;
    let mut secret_content = String::new();
    secret_file.read_to_string(&mut secret_content)?;

    let secret_data: SecretData = from_str(&secret_content)?;
    let id = secret_data.id;
    let password = secret_data.password;

    println!("ID: {}, Password: ", id);

    // Reading stock_id.json
    let mut stock_id_file = File::open("./stock_id.json")?;
    let mut stock_id_content = String::new();
    stock_id_file.read_to_string(&mut stock_id_content)?;

    let data: Vec<Stock> = from_str(&stock_id_content)?;
    for v in data {
        println!("{:?}", v.id);
        println!("{:?}", v.quantity);
    }
    

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
//     let caps = DesiredCapabilities::firefox();
//     let driver = WebDriver::new("http://localhost:4444", caps).await?;
//     // Navigate to https://wikipedia.org.
//     driver.goto("https://wikipedia.org").await?;
//     let elem_form = driver.find(By::Id("search-form")).await?;

//     // Find element from element.
//     let elem_text = elem_form.find(By::Id("searchInput")).await?;

//     // Type in the search terms.
//     elem_text.send_keys("selenium").await?;

//     // Click the search button.
//     let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
//     elem_button.click().await?;

//     // Look for header to implicitly wait for the page to load.
//     driver.query(By::ClassName("firstHeading")).first().await?;
//     assert_eq!(driver.title().await?, "Selenium - Wikipedia");

//     // Always explicitly close the browser.
//     driver.quit().await?;

//     Ok(())
// }