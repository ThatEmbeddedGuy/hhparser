extern crate urlencoding;

pub async fn get_page(_num: i32, _keyword: &str) -> Option<String> {
    let mut request_uri = REQUEST_URI.to_string() + &urlencoding::encode(_keyword);
    if _num != 0 {
        request_uri.push_str(PAGE_PARAM);
        request_uri.push_str(&_num.to_string());
    }
    let client = reqwest::Client::new();
    let res = client
        .get(&request_uri)
        .header(reqwest::header::USER_AGENT, USER_AGENT_CHROME)
        .send()
        .await
        .unwrap()
        .text()
        .await;
    match res {
        Ok(x) => Some(x),
        Err(x) => {
            println!("request::get_page error: {}", x);
            None
        }
    }
}

async fn make_request(url: &String) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    match resp.error_for_status() {
        Ok(_res) => return Ok(_res.text().await?.to_string()),
        Err(err) => Err(err),
    }
}

//TODO use hh api
const REQUEST_URI: &str = "https://api.hh.ru/vacancies?area=2&text=";
const PAGE_PARAM: &str = "&page=";

const USER_AGENT_CHROME: &str = "Mozilla/5.0 (Linux; Android 4.0.4; Galaxy Nexus Build/IMM76B) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.133 Mobile Safari/535.19";
