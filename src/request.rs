extern crate urlencoding;

pub async fn get_page(_num: i32, _keyword: String) -> Option<String> {
    println!("get page method: page - {} ", _num);
    let pages_suffix = if _num == 0 {
        String::new()
    } else {
        String::from(PAGE_PARAM) + &_num.to_string()
    };
    let request_uri: String =
        String::from(REQUEST_URI) + &urlencoding::encode(&_keyword) + &pages_suffix;

    let response = make_request(&request_uri).await?;

    let res = response.text().await;
    match res {
        Ok(x) => Some(x),
        Err(x) => {
            println!("response::text parsing error: {}", x);
            None
        }
    }
}

async fn make_request(url: &str) -> Option<reqwest::Response> {
    let client = reqwest::Client::new();

    let result = client
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT_CHROME)
        .send()
        .await;
    //TODO retry for 408 429 480 5XX
    if let Err(message) = &result {
        println!("request::make_request error: {}", message);
    }

    result.ok()
}

const REQUEST_URI: &str = "https://api.hh.ru/vacancies?area=2&text=";
const PAGE_PARAM: &str = "&page=";

const USER_AGENT_CHROME: &str = "Mozilla/5.0 (Linux; Android 4.0.4; Galaxy Nexus Build/IMM76B) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.133 Mobile Safari/535.19";
