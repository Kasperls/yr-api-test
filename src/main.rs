use exitfailure::ExitFailure;
use reqwest::Url;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_derive::{Serialize, Deserialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct WeatherData {
    s: String,
}

impl WeatherData {
    async fn get() -> Result<reqwest::Response, ExitFailure> {
        let url = format!(
            "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat=60.10&lon=9.58",
        );
        let header = HeaderMap::new();
        header.insert(USER_AGENT, HeaderValue::from_static())
        let uri = Url::parse(&*url)?;
        let res = reqwest::get(uri).await?; //.json().await?;

        Ok(res)
    }
}


#[tokio::main]
async fn main() -> Result<(), ExitFailure> {

    let res = WeatherData::get().await?;
    println!("{:?}", res);

    Ok(()) 
}