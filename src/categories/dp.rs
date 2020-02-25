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
	use std::cmp;
	if prices.is_empty() {return 0}

	#[derive(Copy, Clone)]
	#[derive(Debug)]
	struct Stock {
		b:(i32, i32),
		s:(i32, i32),
		c:(i32, i32)
	}
	let l = prices.len();
	let mut profits = vec![Stock{b:(0, 0), s:(0, 0),  c:(0, 0)}; l];
	profits[0].b = (-prices[0], 1);

	// for i in 1 .. l {
	// 	profits[i].b = cmp::max(profits[i-1].b - prices[i], profits[i-1].c - prices[i]);
	// 	profits[i].c = cmp::max(cmp::max(profits[i-1].b, profits[i-1].s), profits[i-1].c);
	// 	profits[i].s = profits[i].c + prices[i];
	// }
	println!("{:?}", profits);
	return -1 //cmp::max(cmp::max(profits[l-1].b, profits[l-1].s), profits[l-1].c)
}

pub fn word_breaks_last(s: String, word_dict: Vec<String>) -> Vec<String> {
	use std::collections::HashMap;

	fn rec_find(st: &String, wd: &Vec<String>, store: &mut HashMap<String, Vec<String>>) -> Vec<String> {
		let mut results = Vec::new();
		if store.contains_key(st) {
			return store[st].clone();
		}
		if st.is_empty() {
			results.push("".to_string());
		}
		else {
			for w in wd {
				let l = w.chars().count();
				if st.chars().count() >= l && &st[0 .. l] == w {
					let ans = rec_find(&st[l ..].to_string(), wd, store);
					for a in ans {
						if a.is_empty() {
							results.push(w.to_string());
						}
						else {
							results.push(vec![w.to_string(), a].join(" "));
						}
					}
				}
			}
		}
		store.insert(st.to_string(), results.clone());
		return results
	}
	let mut storage = HashMap::new();
	return rec_find(&s, &word_dict, &mut storage);
}

pub fn word_breaks_new(s: String, word_dict: Vec<String>) -> Vec<String> {
	use std::collections::HashSet;
	use std::collections::HashMap;

	let mut dic_set = HashSet::new();
	let mut possible_breaks:HashMap<String, HashSet<String>> = HashMap::new();

	for i in &word_dict {
		dic_set.insert(i.to_string());
	}
	for i in &word_dict {
		let entry = possible_breaks.entry(i.to_string()).or_insert(HashSet::<String>::new());
		entry.insert(i.to_string());
	}
	let l = s.chars().count();

	// println!("{:?}", possible_breaks);

	for d in 2 ..= l {
		for i in 0 .. l - d + 1 {
			for j in i .. i + d - 1 {
				if possible_breaks.contains_key(&s[i .. j + 1]) && possible_breaks.contains_key(&s[j + 1 .. i + d]) {
					let mut tmp = HashSet::new();
					for l in &possible_breaks[&s[i .. j + 1]] {
						for r in &possible_breaks[&s[j + 1 .. i + d]] {
							tmp.insert(vec![l.to_string(), r.to_string()].join(" "));
						}
					}
					let entry = possible_breaks.entry(s[i .. i + d].to_string()).or_insert(HashSet::<String>::new());
					for s in tmp {
						entry.insert(s);
					}
				}
			}
		}
		println!("{}", d);
		// println!("{:?}", possible_breaks);
	}

	if possible_breaks.contains_key(&s) {
		let mut r = Vec::new();
		for e in &possible_breaks[&s] {
			r.push(e.to_string());
		}
		return r;
		// return possible_breaks[&s].collect::<Vec<String>>();
	}
	return vec![];
}

pub fn word_breaks(s: String, word_dict: Vec<String>) -> Vec<String> {
	use std::collections::HashSet;
	use std::collections::HashMap;

	let mut dic_set = HashSet::new();
	for i in &word_dict {
		dic_set.insert(i);
	}
	let l = s.chars().count();
	let mut grid = vec![vec![0 as u64; l]; l];
	let mut possible_breaks:Vec<HashSet<String>> = vec![HashSet::new(); l*l];

	for i in 0 .. l {
		if dic_set.contains(&s[i .. i + 1].to_string()) {
			grid[i][i] = 1;
			possible_breaks[i*l + i].insert(s[i .. i + 1].to_string());
		}
	}

	fn cartesian_prod(lv:&HashSet<String>, rv:&HashSet<String>) -> HashSet<String> {
		let mut results = HashSet::new();
		for l in lv {
			for r in rv {
				results.insert(vec![l.to_string(), r.to_string()].join(" "));
			}
		}
		return results
	}

	// println!("{:?}", possible_breaks);

	for d in 2 ..= l {
		for i in 0 .. l - d + 1 {
			let mut cnt = 0;
			for j in i .. i + d - 1 {
				if grid[i][j] > 0 && grid[j + 1][i + d - 1] > 0 {
					let tmp = cartesian_prod(&possible_breaks[i * l + j], &possible_breaks[l * (j + 1) + i + d - 1]);
					let ut = possible_breaks[i * l + i + d - 1].union(&tmp).map(|x| x.to_string()).collect::<HashSet<String>>();
					possible_breaks[i * l + i + d - 1] = ut;
				}
				cnt += grid[i][j]*grid[j + 1][i + d - 1];
			}
			if dic_set.contains(&s[i .. i + d].to_string()) {
				possible_breaks[i * l + i + d - 1].insert(s[i .. i + d].to_string());
				cnt += 1
			}
			grid[i][i + d - 1] = cnt;
		}
	}

	// for v in &grid {
	// 	println!("{:?}", *v);
	// }

	if grid[0][l-1] > 0 {
		// println!("{:?}", possible_breaks);
		let mut r = Vec::new();
		for e in &possible_breaks[l - 1] {
			r.push(e.to_string());
		}
		return r;
	}
	return vec![];
}

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
	use std::collections::HashSet;
	let mut dic_set = HashSet::new();
	for i in &word_dict {
		dic_set.insert(i);
	}
	let l = s.chars().count();
	// print!("{}",l);
	let mut grid = vec![vec![0 as u64; l]; l];
	for i in 0 .. l {
		if dic_set.contains(&s[i .. i + 1].to_string()) {
			grid[i][i] = 1;
		}
	}
	for d in 2 ..= l {
		for i in 0 .. l - d + 1 {
			let mut cnt = 0;
			for j in i .. i + d - 1 {
				cnt += grid[i][j]*grid[j + 1][i + d - 1];
			}
			if dic_set.contains(&s[i .. i + d].to_string()) {
				cnt += 1
			}
			grid[i][i + d - 1] = cnt;
		}
	}
	// for v in &grid {
	// 	println!("{:?}", *v);
	// }
	if grid[0][l-1] > 0 {
		return true
	}
	return false
}

pub fn max_coins(nums: Vec<i32>) -> i32 {
	let mut p = Vec::new(); 
	p.push(1);
	for i in &nums {
		p.push(*i);
	}
	p.push(1);

	let l = p.len();

	let mut cost = vec![vec![0; l]; l];
	
	for d in 2 .. l {
		for i in 0 .. l - d {
			for k in i + 1 .. i + d {
				let q = cost[i][k] + cost[k][i + d] + p[i] * p[k] * p[i + d];
				if q > cost[i][i + d] {
					cost[i][i + d] = q;
				}
			}
		}
	}
	println!("{:?}", cost);
	return cost[0][l - 1]
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

