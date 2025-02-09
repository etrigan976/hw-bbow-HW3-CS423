fn main() {}
#[test]
fn extend_from_text_test() -> () {
    use bbow::Bbow;
    let bbow = Bbow::new().extend_from_text("Hello world.");
    assert_eq!(2, bbow.len());
    assert_eq!(1, bbow.match_count("hello"));
}
#[test]
fn match_count_test() -> () {
    use bbow::Bbow;
    let bbow = Bbow::new()
            .extend_from_text("b b b-banana b");
    assert_eq!(3, bbow.match_count("b"));
}
#[test]
fn count_test() -> () {
    use bbow::Bbow;
    let bbow = Bbow::new()
        .extend_from_text("Can't stop this! Stop!");
    assert_eq!(3, bbow.count());
}
#[test]
fn len_test() -> () {
    use bbow::Bbow;
    let bbow = Bbow::new()
         .extend_from_text("Can't stop this! Stop!");
     assert_eq!(2, bbow.len());
}
#[test]
fn is_empty_test() -> () {
    use bbow::Bbow;
    let bbow = Bbow::new()
        .extend_from_text("Super Bowl Sunday");
    assert_eq!(false, bbow.is_empty());
}