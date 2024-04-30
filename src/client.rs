use reqwest::{
    header::{self, HeaderMap, HeaderName, HeaderValue},
    Client,
};

use super::model::*;

const BASE_URL: &str = "https://www.faycarsons.xyz/store";

pub async fn get_data(filter: String) -> Result<Users, String> {
    let headers = HeaderMap::from_iter([
        (header::USER_AGENT, HeaderValue::from_static("KiggyMetric")),
        (
            HeaderName::from_static("auth"),
            HeaderValue::from_str(super::evil_env::EVIL_AUTH)
                .map_err(|e| format!("INVALID AUTH :: {e}"))?,
        ),
    ]);
    Client::new()
        .get(format!("{BASE_URL}/{filter}"))
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Error fetching data: {e}"))?
        .json::<Users>()
        .await
        .map_err(|e| format!("Error receiving or deseriallizing response: {e}"))
}

pub async fn send_query(statement: &str) -> Result<Option<Users>, String> {
    let headers = HeaderMap::from_iter([
        (header::CONTENT_TYPE, HeaderValue::from_static("text/plain")),
        (
            HeaderName::from_static("auth"),
            HeaderValue::from_str(super::evil_env::EVIL_AUTH)
                .map_err(|e| format!("INVALID AUTH: {e}"))?,
        ),
        (header::USER_AGENT, HeaderValue::from_static("KiggyMetric")),
    ]);
    let request = reqwest::Client::new()
        .put(format!("{BASE_URL}/admin"))
        .headers(headers)
        .body(statement.to_string())
        .send()
        .await;

    match request {
        Ok(res) => {
            use reqwest::StatusCode;
            match res.status() {
                StatusCode::ACCEPTED | StatusCode::OK => {
                    let body = res.text().await;
                    match body {
                        Ok(data) => match data.as_str() {
                            "" | "[]" => Ok(None),
                            data => {
                                println!("REQWEST RECEIVED DATA");
                                let data = serde_json::from_str(&data);
                                if let Ok(data) = data {
                                    Ok(data)
                                } else {
                                    Err("INVALID RESPONSE BODY".to_string())
                                }
                            }
                        },
                        Err(e) => Err(format!("ERROR IN RESPONSE BODY: {e}")),
                    }
                }
                StatusCode::INTERNAL_SERVER_ERROR => Err(res
                    .text()
                    .await
                    .unwrap_or("UNKNOWN ERROR FROM SERVER".to_owned())),
                status => {
                    let err = res.text().await.unwrap_or("NO ERROR MESSAGE".to_owned());
                    Err(format!("UNHANDLED STATUS CODE {status}: {err}"))
                }
            }
        }
        Err(e) => Err(format!("Cannot execute query: {e}")),
    }
}
