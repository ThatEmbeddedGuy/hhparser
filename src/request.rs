extern crate  urlencoding;

pub async fn get_page(_num: i32, _keyword:  &str ) -> String
{
  let mut request_uri = REQUEST_URI.to_string() + &urlencoding::encode(_keyword) ;
  if _num !=0
  {
    request_uri.push_str(PAGE_PARAM);
    request_uri.push_str(&_num.to_string());
  }
  let res = make_request(&request_uri).await;
  match res { 
  Ok(x) =>  x,
  _ => "".to_string()
  }
}


async fn make_request(url: &String) ->  Result<String, reqwest::Error>
{
    let resp = reqwest::get(url).await?;
    match resp.error_for_status()
    {
             Ok(_res) =>  return Ok(_res.text().await?.to_string()),
             Err(err) => Err(err)
   }
}



const REQUEST_URI : &str = "https://spb.hh.ru/search/vacancy?clusters=true&area=2&enable_snippets=true&salary=&st=searchVacancy&text=";
const PAGE_PARAM : &str = "&page=";