extern crate rust_template;

use rust_template::count_all_stars;

#[test]
pub fn test_universe() {
    let expected = 6;
    let actual = count_all_stars(&[1, 2, 3]);
    assert_eq!(expected, actual);
}
