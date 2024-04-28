use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Invoking the API...");
    dotenv().ok();
    let api_key = dotenv::var("BAD_WORDS_API_KEY").unwrap();
    println!("API Key: {}", api_key);

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", api_key)
        .body("a list with shit words")
        .send()
        .await?
        .text()
        .await?;

    println!("{}", res);

    Ok(())
}
