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

#[test]
#[ignore]
fn test_max_profit() {
	assert_eq!(dp::max_profit(vec![1,2,3,0,2]), 3);
	// assert_eq!(dp::max_profit(vec![]), 0);
}

#[test]
// #[ignore]
fn test_max_coins() {
	assert_eq!(dp::max_coins(vec![3, 1, 5, 8]), 167);
}

#[test]
#[ignore]
fn test_word_break2() {
	// let v = vec!["cat", "cats", "and", "sand", "dog"];
	// let v1 = v.iter().map(|&x| x.to_string()).collect::<Vec<String>>();

	// assert_eq!(dp::word_breaks("catsanddog".to_string(), v1), vec!["cats and dog", "cat sand dog"]);
	assert_eq!(dp::word_breaks("a".to_string(), vec!["a".to_string()]), vec!["a"]);
	assert_eq!(dp::word_breaks("ab".to_string(), vec!["a".to_string(), "b".to_string()]), vec!["a b"]);

	let v = vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"];
	let v1 = v.iter().map(|&x| x.to_string()).collect::<Vec<String>>();

	assert_eq!(dp::word_breaks("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(), v1), Vec::<String>::new());
}

#[test]
// #[ignore]
fn test_word_break3() {
	// let v = vec!["cat", "cats", "and", "sand", "dog"];
	// let v1 = v.iter().map(|&x| x.to_string()).collect::<Vec<String>>();

	// assert_eq!(dp::word_breaks_last("catsanddog".to_string(), v1), vec!["cats and dog", "cat sand dog"]);
	assert_eq!(dp::word_breaks_last("a".to_string(), vec!["a".to_string()]), vec!["a"]);
	assert_eq!(dp::word_breaks_last("ab".to_string(), vec!["a".to_string(), "b".to_string()]), vec!["a b"]);

	let v = vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"];
	let v1 = v.iter().map(|&x| x.to_string()).collect::<Vec<String>>();

	assert_eq!(dp::word_breaks_last("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(), v1), Vec::<String>::new());
}

#[test]
fn test_word_break1() {
	let v = vec!["cats", "dog", "sand", "and", "cat"];
	let v1 = v.iter().map(|&x| x.to_string()).collect::<Vec<String>>();
	assert_eq!(dp::word_break("catsandog".to_string(), v1), false);

	assert_eq!(dp::word_break("applepenapple".to_string(), vec!["apple".to_string(), "pen".to_string()]), true);
	assert_eq!(dp::word_break("leetcode".to_string(), vec!["leet".to_string(), "code".to_string()]), true);

	let v = vec!["cbc","bcda","adb","ddca","bad","bbb","dad","dac","ba","aa","bd","abab","bb","dbda","cb","caccc","d","dd","aadb","cc","b","bcc","bcd","cd","cbca","bbd","ddd","dabb","ab","acd","a","bbcc","cdcbd","cada","dbca","ac","abacd","cba","cdb","dbac","aada","cdcda","cdc","dbc","dbcb","bdb","ddbdd","cadaa","ddbc","babb"];
	let v1 = v.iter().map(|&x| x.to_string()).collect::<Vec<String>>();
	assert_eq!(dp::word_break("bccdbacdbdacddabbaaaadababadad".to_string(), v1), true);

}

fn main() {
    println!("Hello, world!");
}
