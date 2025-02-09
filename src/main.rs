use bbow::Bbow;
fn main() {}
#[test]
fn extend_from_text_test() -> () {
    let bbow = Bbow::new().extend_from_text("Hello world.");
    assert_eq!(2, bbow.len());
    assert_eq!(1, bbow.match_count("hello"));
}