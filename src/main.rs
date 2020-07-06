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
    _keyword: &str,
    _pages: u64,
) -> std::vec::Vec<impl std::future::Future<Output = Option<String>>> {
    (1.._pages)
        .map(|page| request::get_page(page as i32, _keyword.to_string()))
        .collect::<Vec<_>>()
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
    /// fmt - Export format [print|txt|json]
    #[clap(long, default_value = "print")]
    fmt: String,
    /// filename - Export filename, used in txt/json format
    #[clap(short, long, default_value = "export.txt")]
    filename: String,
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
            let pages_total = parser::parse_num_of_pages(&first);
            let first_page_vacancies = parser::parse_vacancies_json(&first);
            println!(
                "First page parsed: num of pages: {} , vacancies: {}",
                pages_total,
                first_page_vacancies.len()
            );

            // Get rest index pages simultaneously, filter invalid ones
            let _tasks = get_rest_pages(&search_keyword, pages_total).await;
            let _index_pages: std::vec::Vec<String> = futures::future::join_all(_tasks)
                .await
                .into_iter()
                .filter_map(|x| x)
                .collect();

            //Parse all vacancies from all pages, flatten all vacancies in one list, filter invalid ones while parsing
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
                (String::from("id"), vacancy.id),
                (String::from("url"), vacancy.url),
                (String::from("name"), vacancy.name),
                (String::from("description"), vacancy.snippet),
                (
                    String::from("salary from"),
                    vacancy
                        .salary_from
                        .map(|x| x.to_string())
                        .unwrap_or_default(),
                ),
                (
                    String::from("salary to"),
                    vacancy.salary_to.map(|x| x.to_string()).unwrap_or_default(),
                ),
                (
                    String::from("salary gross"),
                    vacancy.salary_gross.to_string(),
                ),
            ]
            .iter()
            .cloned()
            .collect()
        })
        .collect::<Vec<_>>();
    export::export(&opts.fmt, &opts.filename, &vacancies_key_value);

    println!("Finished");
}
