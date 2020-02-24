#[allow(non_camel_case_types)]
pub struct maxprodsum;

impl maxprodsum {
	pub fn max_product(nums: Vec<i32>) -> i32 {
		use std::cmp;
		let mut last:(i32, i32) = (nums[0], nums[0]);
		let mut max_prod:i32 = nums[0];
		for i in &nums[1..] {
			last = (cmp::max(cmp::max(*i * last.0, *i * last.1), *i), cmp::min(cmp::min(*i * last.0, *i * last.1), *i));
			max_prod = cmp::max(max_prod, cmp::max(last.0, last.1));
		}
		return max_prod;
	}
}

pub fn num_decodings(s: String) -> i32 {
	let l = s.chars().count();
	// println!("{}, {}", s, l);

	let mut no_decs = vec![0; l+1];
	if s[0 .. 1] != "0".to_string() {
		no_decs[0] = 1;
		no_decs[1] = 1;
	}
	for i in 1 .. l {
		let tmp: i32 = s[i .. i+1].parse().unwrap_or(0);
		if tmp > 0 {
			no_decs[i+1] += no_decs[i];
		}
		let prev: i32 = s[i-1 .. i].parse().unwrap_or(0);
		if prev > 0 && (10*prev + tmp) <= 26 {
			no_decs[i+1] += no_decs[i-1];
		}
	}
	return no_decs[l]
}

pub fn  num_squares(n: i32) -> i32 {
	use std::cmp;
	let mut sqrs:Vec<i32> = (0 .. n+1).collect();
	// let mut sqrs = vec![0; n as usize + 1];
	let mut k = 1;
	for i in 1 .. n as usize + 1 {
		if i == k*k {
			sqrs[i] = 1;
			k += 1;
		}
		else {
			let mut low_bound = (i as f32).sqrt().sqrt() as usize;
			while low_bound <= (k - 1) {
				sqrs[i] = cmp::min(sqrs[i], sqrs[low_bound * low_bound] + sqrs[i - low_bound * low_bound]);
				low_bound += 1
			}
		}
	}
	// println!("{:?}", sqrs);
	return sqrs[n as usize]
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
	println!("{:?}", prices);
	return -1
}

pub fn word_breaks(s: String, word_dict: Vec<String>) -> Vec<String> {
	println!("{:?}, {}", word_dict, s);
	return vec!["write".to_string(), "something".to_string()]
}

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
	println!("{:?}, {}", word_dict, s);
	return false
}

pub fn max_coins(nums: Vec<i32>) -> i32 {
	println!("{:?}", nums);
	return 32
}

// medium problems
pub fn can_jump(nums: Vec<i32>) -> bool {
	use std::cmp;
	// println!("{:?}", nums);
	let l = nums.len() as usize;
	let mut reachable = vec![false; l];
	reachable[0] = true;

	for i in 0 .. l {
		if reachable[i] == true {
			for j in 1 ..= nums[i] {
				let r = i + j as usize;
				if r >= l { break; }
				reachable[r] = true;
				if l == r { return true; }
			}
		}
	}
	return reachable[l - 1]
}

pub fn unique_paths(m: i32, n: i32) -> i32 {
	// println!("{}, {}", m, n);
	let mut grid:Vec<Vec<i32>> = vec![vec![0; n as usize + 1]; m as usize + 1];
	grid[1][0] = 1;
	for i in 1 ..= m as usize {
		for j in 1 ..= n as usize {
			grid[i][j] = grid[i - 1][j] + grid[i][j-1]
		}
	}
	return grid[m as usize][n as usize];
}

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
	use std::cmp;
	// println!("{:?}, {}", coins, amount);
	let mut denoms = vec![::std::i32::MAX; amount as usize + 1];
	denoms[0] = 0;
	for i in 0 ..= amount {
		for l in &coins {
			if i - *l >= 0 {
				denoms[i as usize] = cmp::min(denoms[i as usize] as i64, denoms[i as usize - *l as usize] as i64 + 1) as i32;
			}
		}
	}
	// println!("{:?}", denoms);
	if denoms[amount as usize] == ::std::i32::MAX {
		return -1
	}
	else { return denoms[amount as usize] }
}

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
	if nums.is_empty() {
		return 0
	}
	#[derive(Copy, Clone)]
	#[derive(Debug)]
	struct LIS {
		length:i32,
		index:usize
	}

	let l = nums.len();
	let mut maxlen = 1;
	// let mut end_idx = 0;
	let mut lis_array:Vec<LIS> = vec![LIS{length:1, index:0}; l];
	lis_array[0].index = 0;
	for i in 1 .. l {
		lis_array[i].index = i;
		for j in 0 .. i {
			if lis_array[i].length <= lis_array[j].length && nums[j] < nums[i] {
				lis_array[i].length = lis_array[j].length + 1;
				lis_array[i].index = j;
			}
		}
		if lis_array[i].length > maxlen {
			maxlen = lis_array[i].length;
			// end_idx = i;
		}
	}
	
	// let mut ans: Vec<i32> = Vec::new();
	// while end_idx != lis_array[end_idx].index {
	// 	ans.push(nums[end_idx]);
	// 	end_idx = lis_array[end_idx].index;
	// }
	// ans.push(nums[end_idx]);
	// ans.reverse();

	return maxlen;
}

