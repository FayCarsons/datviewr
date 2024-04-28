use reqwest::header::HeaderValue;

use super::model::*;

const BASE_URL: &str = "https://www.faycarsons.xyz/store";

pub async fn get_data(filter: String) -> Result<Users, String> {
    let client = reqwest::Client::new()
        .get(format!("{BASE_URL}/{filter}"))
        .header(
            reqwest::header::USER_AGENT,
            HeaderValue::from_static("KiggyMetric"),
        );
    client
        .send()
        .await
        .map_err(|e| format!("Error fetching data: {e}"))?
        .json::<Users>()
        .await
        .map_err(|e| format!("Error receiving or deseriallizing response: {e}"))
}

pub async fn send_query(statement: &str) -> Result<(), String> {
    use reqwest::header::*;
    let headers = HeaderMap::from_iter([
        (CONTENT_TYPE, HeaderValue::from_static("text/plain")),
        (
            HeaderName::from_static("auth"),
            HeaderValue::from_str(super::evil_env::EVIL_AUTH)
                .map_err(|e| format!("INVALID AUTH: {e}"))?,
        ),
        (USER_AGENT, HeaderValue::from_static("KiggyMetric")),
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
                StatusCode::ACCEPTED | StatusCode::OK => Ok(()),
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
