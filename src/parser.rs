extern crate serde_json;

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
    if let Some(root) = parse_json(_body)
    {
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
    assert_eq!(get_num_of_pages(""),0);
}

#[test]
fn correct_json() {
    assert_eq!(get_num_of_pages("{\"pages\": 11}"),11);
}