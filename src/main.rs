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
        .and_then(parser::into_json)
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
        //Get first page synchronously, just to get number of pages
        if let Some(first) = get_first_page(&search_keyword).await {
            let pages = parser::parse_num_of_pages(&first);
            let first_page_vacancies = parser::parse_vacancies_json(&first);
            println!(
                "First page parsed: num of pages: {} , vacancies: {}",
                pages,
                first_page_vacancies.len()
            );

            // Get rest index pages simultaneously
            let _tasks = get_rest_pages(&search_keyword, pages).await;
            let _index_pages: std::vec::Vec<String> = futures::future::join_all(_tasks)
                .await
                .into_iter()
                .filter_map(|x| x)
                .collect();

            //Parse all vacancies from all pages, flatten all vacancies in one list, filter invalid
            let rest_pages_vacancies = _index_pages
                .into_iter()
                .map(parser::parse_vacancies_string)
                .flatten()
                .collect::<Vec<_>>();

            //TODO filter already cached vacancies
            all_vacancies.extend(first_page_vacancies.into_iter());
            all_vacancies.extend(rest_pages_vacancies.into_iter());
        }
    }
    // Convert vector of parsed structures into vector of  generic key/value maps
    // Export shouldn't rely on vacancy structure
    let vacancies_key_value = all_vacancies
        .into_iter()
        .map(|vacancy| {
            [
                ("ID".to_string(), vacancy.id),
                ("url".to_string(), vacancy.url),
                ("name".to_string(), vacancy.name),
                ("Description".to_string(), vacancy.snippet),
                (
                    "from".to_string(),
                    vacancy
                        .salary_from
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                ),
                (
                    "to".to_string(),
                    vacancy.salary_to.map(|x| x.to_string()).unwrap_or_default(),
                ),
                ("gross".to_string(), vacancy.salary_gross.to_string()),
            ]
            .iter()
            .cloned()
            .collect()
        })
        .collect::<Vec<_>>();

    export::export(&opts.fmt, vacancies_key_value);

    println!("Finished");
}
