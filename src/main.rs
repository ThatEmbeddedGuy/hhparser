extern crate futures;
extern crate reqwest;
extern crate tokio;
extern crate tokio_io;

mod parser;
mod request;

const TITLE: &str = "C++";

async fn get_first_page(_keyword: &str) -> Option<serde_json::Value> {
    request::get_page(0, _keyword.to_string())
        .await
        .and_then(parser::parse_json_own)
}

async fn get_rest_pages(
    _keyword: &String,
    _pages: u64,
) -> std::vec::Vec<impl std::future::Future<Output = Option<String>>> {
    let index_pages = {
        let mut index_pages_futures = Vec::new();
        let mut page = 0;
        index_pages_futures.resize_with(_pages as usize, || {
            page += 1;
            request::get_page(page,  _keyword.clone())
        });
        index_pages_futures
    };

    index_pages
}

#[tokio::main]
async fn main() {
    println!("Started");
    // TODO parse CL arguments (get, clean DB, export)
    if let Some(first) = get_first_page(TITLE).await {
        let pages = parser::parse_num_of_pages(&first);
        println!("num of pages parsed: {}", pages);
        let _tasks = get_rest_pages(&TITLE.to_string(), pages).await;
        let _index_pages = futures::future::join_all(_tasks).await;
        //TODO filter already cached vacancies
        //TODO perform requests for new vacancies
    }

    println!("Finished");
}
