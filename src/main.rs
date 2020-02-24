#[allow(unused_imports)]

pub mod categories;
use categories::dp;

#[test]
fn test_max_prod_array() {
	let v = vec![2,3,-2,4];
	assert_eq!(dp::maxprodsum::max_product(v), 6);

	let v = vec![-2,0,-1];
	assert_eq!(dp::maxprodsum::max_product(v), 0);
}

#[test]
fn test_no_decodings() {
	assert_eq!(dp::num_decodings("12".to_string()), 2);
	assert_eq!(dp::num_decodings("226".to_string()), 3);
	assert_eq!(dp::num_decodings("0".to_string()), 0);
	assert_eq!(dp::num_decodings("011".to_string()), 0);
	assert_eq!(dp::num_decodings("100".to_string()), 0);
	assert_eq!(dp::num_decodings("101".to_string()), 1);
}

#[test]
fn test_prefect_square() {
	assert_eq!(dp::num_squares(12), 3);
	assert_eq!(dp::num_squares(13), 2);
}

#[test]
fn test_can_jump() {
	assert_eq!(dp::can_jump(vec![2,3,1,1,4]), true);
	assert_eq!(dp::can_jump(vec![3,2,1,0,4]), false);
}

#[test]
fn test_unique_paths() {
	assert_eq!(dp::unique_paths(3, 2), 3);
	assert_eq!(dp::unique_paths(7, 3), 28);
}

#[test]
fn test_coin_change() {
	assert_eq!(dp::coin_change(vec![1, 2, 5], 11), 3);
	assert_eq!(dp::coin_change(vec![2], 3), -1);
}

#[test]
fn test_length_of_lis() {
	assert_eq!(dp::length_of_lis(vec![10,9,2,5,3,7,101,18]), 4);
	assert_eq!(dp::length_of_lis(vec![]), 0);
}

fn main() {
    println!("Hello, world!");
}
