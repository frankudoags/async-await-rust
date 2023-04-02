use std::io::Write;

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://www.rust-lang.org";
    let res = reqwest::get(url).await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    //create a file index.html
    let mut file = std::fs::File::create("index.html")?;
    //write the body to the file
    file.write_all(body.as_bytes())?;
    Ok(())
}
