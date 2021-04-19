use reqwest::Error;

pub async fn sendPush(body: &str) -> Result<(), Error> {
    let response = reqwest::get("http://116.202.19.18:90/profile/").await?;
    println!("Response Code: {}, Body: {}", response.status().to_string(), response.text().await?);
    Ok(())
}