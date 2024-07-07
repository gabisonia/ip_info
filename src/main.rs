use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct IpInfo {
    ip: String,
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    org: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let response = reqwest::get("https://ipinfo.io/json").await?;

    let ip_info: IpInfo = response.json().await?;

    println!("IP Address: {}", ip_info.ip);
    if let Some(city) = ip_info.city {
        println!("City: {}", city);
    }
    if let Some(region) = ip_info.region {
        println!("Region: {}", region);
    }
    if let Some(country) = ip_info.country {
        println!("Country: {}", country);
    }
    if let Some(org) = ip_info.org {
        println!("Organization: {}", org);
    }

    Ok(())
}