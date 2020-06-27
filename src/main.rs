use reqwest;
use std::fs::File;
use std::io::prelude::*;

static URL: &str = "https://www.immoweb.be/en/search/house-and-apartment/for-rent/reet/2840?countries=BE&orderBy=relevance";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Request to Url: [{}]", URL);
    let resp = reqwest::blocking::get(URL)?;
    println!("Response status: [{}]", resp.status());

    let page = resp.text()?;
    let mut file = File::create("response.html")?;
    file.write_all(page.as_bytes())?;
    Ok(())
}