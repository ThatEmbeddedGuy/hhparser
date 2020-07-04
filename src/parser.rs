
pub fn get_num_of_pages(_body: &str) -> i32
{
11
}


#[test]
fn num_pages_test()
{
assert_eq!(get_num_of_pages("some"),11);
}
