use std::ffi::OsString;
use std::process::ExitCode;

// bnum seems to be slightly faster than ruint,
// and 3x faster than uint.
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

fn superscript(number: &str) -> String {
	number
		.chars()
		.map(|c| match c {
			'0' => '⁰',
			'1' => '¹',
			'2' => '²',
			'3' => '³',
			'4' => '⁴',
			'5' => '⁵',
			'6' => '⁶',
			'7' => '⁷',
			'8' => '⁸',
			'9' => '⁹',
			_ => c,
		})
		.collect()
}

fn main() -> ExitCode {
	let args: Vec<OsString> = std::env::args_os().collect();
	if args.len() > 2 {
		eprintln!("Please provide at most 1 argument (power of 10 to search up to).");
		return ExitCode::FAILURE;
	}
	let power_of_10: Option<usize> = match args.get(1) {
		Some(s) => s.clone().into_string().ok().and_then(|x| x.parse().ok()),
		None => Some(35),
	};
	let Some(power_of_10) = power_of_10 else {
		eprintln!("Argument must be a nonnegative integer");
		return ExitCode::FAILURE;
	};
	if power_of_10 > usize::MAX / 4
		|| (power_of_10 as f64 * f64::log2(10.0)) as usize + 10 > size_of::<UInt>() * 8
	{
		eprintln!("Power of 10 is too large for integer type. You will have to increase the size of UInt in the source code.");
		return ExitCode::FAILURE;
	}

	let mut pascal_row = [UInt::from(0u8); 500];
	let mut range = pascal_row.len();
	println!(
		"searching up to 10{}",
		superscript(&format!("{power_of_10}"))
	);
	let limit = UInt::from(10u8).pow(power_of_10 as u32);
	let mut numbers: Vec<UInt> = Vec::new();
	pascal_row[0] = UInt::from(1u8);
	for row in 1u128.. {
		for i in (1..range).rev() {
			pascal_row[i] += pascal_row[i - 1];
			if i > 4 && i as u128 <= row / 2 {
				if is_choose_r(pascal_row[i], 2)
					|| is_choose_r(pascal_row[i], 3)
					|| is_choose_r(pascal_row[i], 4) {
					println!("{}",pascal_row[i]);
				}
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
			if occurrences > 1 {
				println!("{prev}: {occurrences}");
			}
			occurrences = 1;
		}
		prev = n;
	}
	ExitCode::SUCCESS
}
