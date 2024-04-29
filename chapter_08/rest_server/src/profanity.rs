use reqwest_middleware::ClientBuilder;
use reqwest_retry::{
    policies::ExponentialBackoff, RetryTransientMiddleware,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWordsResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

pub async fn check_profanity(
    content: String,
) -> Result<String, error_handlers::Error> {
    let retry_policy =
        ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        // Trace HTTP requests. See the tracing crate to make use of these traces.
        // Retry failed requests.
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    let api_key = dotenv::var("BAD_WORDS_API_KEY")
        .expect("BAD_WORDS_API_KEY must be set");
    println!("API Key: {}", api_key);

    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", api_key)
        .body(content)
        .send()
        .await
        .map_err(|e| {
            error_handlers::Error::MiddlewareReqwestAPIError(e)
        })?;

    if !res.status().is_success() {
        if res.status().is_client_error() {
            let err = transform_error(res).await;
            return Err(error_handlers::Error::ClientError(err));
        } else {
            let err = transform_error(res).await;
            return Err(error_handlers::Error::ServerError(err));
        }
    }

    match res.json::<BadWordsResponse>().await {
        Ok(res) => Ok(res.censored_content),
        Err(e) => Err(error_handlers::Error::ReqwestAPIError(e)),
    }
}

async fn transform_error(
    res: reqwest::Response,
) -> error_handlers::APILayerError {
    error_handlers::APILayerError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}
