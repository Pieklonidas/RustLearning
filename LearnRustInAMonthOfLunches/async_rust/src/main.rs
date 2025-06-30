use tokio;
use reqwest;

async fn async_give_8() -> u8 {
    8
}

#[tokio::main]
async fn main() {
    // let client = reqwest::blocking::Client::default();
    // let response = client.get("https://www.rust-lang.org").send().unwrap();
    // println!("{}", response.text().unwrap());

    let some_number = async_give_8().await;
    let second_number = async_give_8().await;
    println!("{some_number}, {second_number}");

    let client = reqwest::Client::default();
    let response = client
        .get("https://www.rust-lang.org")
        .send()
        .await
        .unwrap();
    println!("{}", response.text().await.unwrap());
}
