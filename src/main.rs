extern crate clap;
extern crate futures;
extern crate reqwest;
extern crate tokio;
extern crate tokio_io;

use clap::Clap;

mod export;
mod parser;
mod request;

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
            request::get_page(page, _keyword.clone())
        });
        index_pages_futures
    };

    index_pages
}

/// headhunter parser
#[derive(Clap)]
#[clap(version = "0.1", author = "Kirill Tikhonov  <yaslimline@gmail.com>")]
struct Opts {
    /// keyword - Search keyword
    #[clap(short, long, default_value = "C++")]
    keyword: String,
    /// export_only - omit search
    #[clap(short, long)]
    export_only: bool,
    /// fmt - Export format (default - print)
    #[clap(short, long, default_value = "print")]
    fmt: String,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    println!("Started");
    //TODO restore vacancies from cache
    let mut all_vacancies = Vec::new();
    
    if !opts.export_only {
        let search_keyword = opts.keyword;
        if let Some(first) = get_first_page(&search_keyword).await {
            let pages = parser::parse_num_of_pages(&first);
            println!("num of pages parsed: {}", pages);
            let _tasks = get_rest_pages(&search_keyword, pages).await;
            let _index_pages: std::vec::Vec<String> = futures::future::join_all(_tasks)
                .await
                .into_iter()
                .filter_map(|x| x)
                .collect();

            let vacancies = _index_pages
                .into_iter()
                .map(parser::parse_vacancies_from_string)
                .flatten()
                .collect::<Vec<_>>();

            //TODO filter already cached vacancies
            all_vacancies  = vacancies;
        }
    }

    export::export(&opts.fmt,all_vacancies );

    println!("Finished");
}
