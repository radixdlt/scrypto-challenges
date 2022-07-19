use reqwest::*;
use serde_json::from_str;

static API1_HOST : &'static str = "https://showcase.api.linx.twenty57.net/UnixTime/tounix?date=now";

pub fn get_time() -> String {
    let result = fetch_time();
    println!("FETCH CURRENT TIME, RESULT: {:?}", result);
    assert!(result.is_ok());

    let time = result.ok().unwrap();

    let time: String = from_str(&time).unwrap();

    return time

}

#[tokio::main]
pub async fn fetch_time() -> Result<String> {

    let response = reqwest::get(API1_HOST).await?;

    let handle = handler(response);

    Ok(handle.await?)

}

pub async fn handler(response: Response) -> Result<String> {
    match response.status() {
        StatusCode::OK => {
             let body = response.text().await?;
            return Ok(body);
        },
        StatusCode::INTERNAL_SERVER_ERROR => {
            panic!("Internal Server Error");
        }
        StatusCode::SERVICE_UNAVAILABLE => {
            panic!("Service Unavailable");
        }
        StatusCode::UNAUTHORIZED => {
            panic!("Unauthorized");
        }
        StatusCode::BAD_REQUEST => {
            panic!("{}", format!("Bad Request: {:?}", response));
        }
        s => {
            panic!("{}", format!("Received response: {:?}", s));
        }
    };
}

