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

pub fn parse_vacancies_from_string(data: String) -> std::vec::Vec<Vacancy> {
    let json = parse_json_own(data).unwrap_or_default();
    parse_vacancies_json(&json)
}

pub fn parse_vacancies_json(item: &serde_json::Value) -> std::vec::Vec<Vacancy> {
    match item["items"].as_array() {
        Some(items) => items.into_iter().filter_map(parse_vacancy_json).collect(),
        None => Vec::new(),
    }
}

fn parse_vacancy_json(item: &serde_json::Value) -> Option<Vacancy> {
    if let Some(salary) = item["salary"].as_object() {
        let vac = Vacancy {
            id: item["id"].as_str().unwrap_or_default().to_string(),
            name: item["name"].as_str().unwrap_or_default().to_string(),
            salary_from: salary["from"].as_u64(),
            salary_to: salary["to"].as_u64(),
            salary_currency: salary["currency"].as_str().unwrap_or_default().to_string(),
            salary_gross: salary["gross"].as_bool().unwrap_or_default(),
            url: item["url"].as_str().unwrap_or_default().to_string(),
            snippet: item["snippet"].as_object().unwrap()["requirement"] // FIXME unwarp may panic with invalid json
                .as_str()
                .unwrap_or_default()
                .to_string(),
            full_description: None,
        };
        return Some(vac);
    }
    return None;
}

#[allow(dead_code)]
pub fn parse_json_own(_body: String) -> Option<serde_json::Value> {
    serde_json::from_str(&_body).ok()
}

#[allow(dead_code)]
pub fn parse_json(_body: &str) -> Option<serde_json::Value> {
    serde_json::from_str(&_body).ok()
}

#[allow(dead_code)]
pub fn parse_num_of_pages(root: &serde_json::Value) -> u64 {
    if let Some(value) = root.get("pages").and_then(|node| node.as_u64()) {
        value
    } else {
        0
    }
}

#[allow(dead_code)]
pub fn get_num_of_pages(_body: &str) -> u64 {
    if let Some(root) = parse_json(_body) {
        parse_num_of_pages(&root)
    } else {
        0
    }
}

#[test]
fn invalid_json() {
    assert_eq!(get_num_of_pages("some"), 0);
}

#[test]
fn empty_string() {
    assert_eq!(get_num_of_pages(""), 0);
}

#[test]
fn correct_json() {
    assert_eq!(get_num_of_pages("{\"pages\": 11}"), 11);
}

#[test]
fn pase_vacancy() {
    let test_data = include_str!("test_one_item.json");
    let json = serde_json::from_str(test_data).unwrap();

    let vac = Vacancy {
        id: "1".to_string(),
        name: "2".to_string(),
        salary_from: Some(1),
        salary_to: None,
        salary_currency: "BYR".to_string(),
        salary_gross: true,
        url: "https://api.hh.ru/vacancies/41".to_string(),
        snippet: "Наши преимущества:".to_string(),
        full_description: None,
    };

    assert_eq!(parse_vacancy_json(&json).unwrap(), vac);
}

#[test]
fn pase_vacancies() {
    let test_data = include_str!("test_full.json");
    let json = serde_json::from_str(test_data).unwrap();

    let vac = Vacancy {
        id: "1".to_string(),
        name: "2".to_string(),
        salary_from: Some(1),
        salary_to: None,
        salary_currency: "BYR".to_string(),
        salary_gross: true,
        url: "https://api.hh.ru/vacancies/41".to_string(),
        snippet: "Наши преимущества:".to_string(),
        full_description: None,
    };

    assert_eq!(parse_vacancies_json(&json)[0], vac);
}
