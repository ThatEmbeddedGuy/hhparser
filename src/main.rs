extern crate tokio;
extern crate tokio_io;
extern crate reqwest;

mod parser;
mod request;

const TITLE: &str = "C++";


async fn parse_hh()
{
  parser::get_num_of_pages("dd");
  print!("{}",request::get_page(0,TITLE).await);
}


#[tokio::main]
async fn main() {
  parse_hh().await;
}