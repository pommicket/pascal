type UInt = bnum::types::U256;

fn is_choose_r(mut k: UInt, r: u32) -> bool {
	// multiply k by r! to avoid future divisions
	for i in 1..=r {
		k *= UInt::from(i);
	}
	let k = k; // no morge changing k
	let log = k.ilog2();
	let mut hi = UInt::from(1u8) << (log / r + 2);
	let mut lo = UInt::from(r + 1);
	'outer: while lo < hi {
		let mid: UInt = (lo + hi) / UInt::from(2u8);
		let mut i = mid;
		let mut mid_falling_r = mid;
		for _ in 1..r {
			i -= UInt::from(1u8);
			mid_falling_r *= i;
			if mid_falling_r > k {
				hi = mid;
				continue 'outer;
			}
		}
		if mid_falling_r < k {
			lo = mid + UInt::from(1u8);
		} else {
			return true;
		}
	}
	false
}

fn main() {
	let mut pascal_row = [UInt::from(0u8); 500];
	let mut range = pascal_row.len();
	let limit = UInt::from(10u8).pow(40);
	let mut numbers: Vec<UInt> = Vec::new();
	pascal_row[0] = UInt::from(1u8);
	for row in 1.. {
		for i in (1..range).rev() {
			pascal_row[i] += pascal_row[i - 1];
			if i > 4 && i <= row / 2 {
				numbers.push(pascal_row[i]);
			}
			if pascal_row[i] > limit {
				range = i;
			}
		}
		if range <= 4 {
			break;
		}
	}
	println!(
		"memory needed = {}MiB",
		(numbers.len() * size_of::<UInt>()) >> 20
	);
	numbers.sort();
	let mut prev = UInt::from(0u8);
	let mut occurrences = 0;
	for n in numbers {
		if n == prev {
			occurrences += 1;
		} else if occurrences > 0 {
			if is_choose_r(prev, 2) {
				occurrences += 1;
			}
			if is_choose_r(prev, 3) {
				occurrences += 1;
			}
			if is_choose_r(prev, 4) {
				occurrences += 1;
			}
			if occurrences > 1 {
				println!("{prev}: {occurrences}");
			}
			occurrences = 1;
		}
		prev = n;
	}
	//sufficiently_small();
}

#[allow(unused)]
fn sufficiently_small() {
	for r in 2u128..100 {
		let fact: f64 = (1..=r).map(|x| x as f64).product();
		println!(
			"r = {r}, largest counterexample = {:?}",
			(r..10000)
				.filter(|&x| {
					2 * x - r + 1
						!= (((x - r + 1..=x).map(|x| x as f64).product::<f64>())
							.powf(1.0 / r as f64) * 2.0)
							.ceil() as u128
				})
				.max()
		);
	}
}
