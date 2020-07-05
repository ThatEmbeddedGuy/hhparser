extern crate reqwest;
extern crate tokio;
extern crate tokio_io;

mod parser;
mod request;

const TITLE: &str = "C++";

async fn get_first_page() -> Option<serde_json::Value>
{
    request::get_page(0, TITLE)
        .await
        .and_then(parser::parse_json_own)
}


async fn get_rest_pages(_pages:u64) -> std::vec::Vec<impl std::future::Future> {

    let index_pages = {
        let mut index_pages_futures = Vec::new();
        let mut page = 0;
        index_pages_futures.resize_with(_pages as usize, || {
            page += 1;
            request::get_page(page, TITLE)
        });
        index_pages_futures
    };

    index_pages
}

#[tokio::main]
async fn main() {
    println!("Started");
    let first = get_first_page().await;
    let pages = parser::parse_num_of_pages(&first.unwrap_or_default());
    println!("num of pages: {}", pages);
    let _tasks = get_rest_pages(pages);
    //TODO pass tasks to thread pool 
    //TODO filter already cached vacancies
    //TODO perform requests for new vacancies
    println!("Finished");
}
