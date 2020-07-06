extern crate serde_json;

//TODO use serde deserialze when it will resolve the conflict with clap

#[derive(Debug, PartialEq)]
pub struct Vacancy {
    pub id: String,
    pub name: String,
    pub salary_from: Option<u64>,
    pub salary_to: Option<u64>,
    pub salary_currency: String,
    pub salary_gross: bool,
    pub url: String,
    pub snippet: String,
    pub full_description: Option<String>,
}

impl std::fmt::Display for Vacancy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id : {}, name:  {} , snippet {})",
            self.id, self.name, self.snippet
        )
    }
}

pub fn parse_vacancies_string(data: String) -> std::vec::Vec<Vacancy> {
    let json = into_json(data).unwrap_or_default();
    parse_vacancies_json(&json)
}

pub fn parse_vacancies_json(item: &serde_json::Value) -> std::vec::Vec<Vacancy> {
    match item["items"].as_array() {
        Some(items) => items.iter().filter_map(parse_vacancy_json).collect(),
        None => Vec::new(),
    }
}

fn parse_vacancy_json(item: &serde_json::Value) -> Option<Vacancy> {
    let (to, from, currency, gross) = match item["salary"].as_object() {
        Some(x) => (
            x["from"].as_u64(),
            x["to"].as_u64(),
            x["currency"].as_str().unwrap_or_default().to_string(),
            x["gross"].as_bool().unwrap_or_default(),
        ),
        _ => (None, None, String::from(""), true),
    };

    let vac = Vacancy {
        id: item["id"].as_str().unwrap_or_default().to_string(),
        name: item["name"].as_str().unwrap_or_default().to_string(),
        salary_from: to,
        salary_to: from,
        salary_currency: currency,
        salary_gross: gross,
        url: item["url"].as_str().unwrap_or_default().to_string(),
        snippet: item["snippet"].as_object().unwrap()["requirement"] // FIXME unwarp may panic with invalid json
            .as_str()
            .unwrap_or_default()
            .to_string(),
        full_description: None,
    };
    Some(vac)
}

pub fn into_json(_body: String) -> Option<serde_json::Value> {
    serde_json::from_str(&_body).ok()
}

#[allow(dead_code)]
pub fn to_json(_body: &str) -> Option<serde_json::Value> {
    serde_json::from_str(&_body).ok()
}

pub fn parse_num_of_pages(root: &serde_json::Value) -> u64 {
    if let Some(value) = root.get("pages").and_then(|node| node.as_u64()) {
        value
    } else {
        0
    }
}

#[allow(dead_code)]
pub fn parse_num_of_pages_str(_body: &str) -> u64 {
    if let Some(root) = to_json(_body) {
        parse_num_of_pages(&root)
    } else {
        0
    }
}

#[test]
fn invalid_json() {
    assert_eq!(parse_num_of_pages_str("some"), 0);
}

#[test]
fn empty_string() {
    assert_eq!(parse_num_of_pages_str(""), 0);
}

#[test]
fn correct_json() {
    assert_eq!(parse_num_of_pages_str("{\"pages\": 11}"), 11);
}

#[test]
fn parse_vacancy() {
    let test_data = include_str!("test_one_item.json");
    let json = serde_json::from_str(test_data).unwrap();

    let vac = Vacancy {
        id: String::from("1"),
        name: String::from("2"),
        salary_from: Some(1),
        salary_to: None,
        salary_currency: String::from("BYR"),
        salary_gross: true,
        url: String::from("https://api.hh.ru/vacancies/41"),
        snippet: String::from("Наши преимущества:"),
        full_description: None,
    };

    assert_eq!(parse_vacancy_json(&json).unwrap(), vac);
}

#[test]
fn parse_vacancies() {
    let test_data = include_str!("test_full.json");
    let json = serde_json::from_str(test_data).unwrap();

    let vac = Vacancy {
        id: String::from("1"),
        name: String::from("2"),
        salary_from: Some(1),
        salary_to: None,
        salary_currency: String::from("BYR"),
        salary_gross: true,
        url: String::from("https://api.hh.ru/vacancies/41"),
        snippet: String::from("Наши преимущества:"),
        full_description: None,
    };

    assert_eq!(parse_vacancies_json(&json)[0], vac);
}
