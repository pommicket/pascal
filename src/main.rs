// LICENSE: WTFPL
use num_bigint::BigUint;
use num_integer::{Integer, Roots};
use std::cmp::min;
use std::ffi::OsString;
use std::process::ExitCode;

// bnum seems to be slightly faster than ruint,
// and 3x faster than uint.
type UInt = bnum::types::U256;

fn is_square(n: UInt) -> bool {
	let sq = n.sqrt();
	sq * sq == n
}

fn is_choose2(mut n: UInt) -> bool {
	n <<= 3;
	n.inc();
	is_square(n)
}

fn find_choose_r(mut k: UInt, r: u32) -> Option<UInt> {
	// multiply k by r! to avoid future divisions
	for i in 1..=r {
		k *= UInt::from(i);
	}
	let k = k; // no morge changing k
	let log = k.ilog2();
	let mut hi = UInt::from(1u8) << (log / r + 2);
	let mut lo = UInt::from(r + 1);
	'outer: while lo < hi {
		let mid: UInt = (lo + hi) >> 1;
		let mut i = mid;
		let mut mid_falling_r = mid;
		for _ in 1..r {
			i.dec();
			mid_falling_r *= i;
			if mid_falling_r > k {
				hi = mid;
				continue 'outer;
			}
		}
		if mid_falling_r < k {
			lo = mid;
			lo.inc();
		} else {
			return Some(mid);
		}
	}
	None
}

fn is_choose_r(k: UInt, r: u32) -> bool {
	find_choose_r(k, r).is_some()
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

#[derive(Clone, Copy)]
struct PascalEntry {
	// row choose col mod 2^64
	value: u64,
	row_lo: u16,
	row_hi: u32,
	col: u16,
}

impl PascalEntry {
	fn new(value: u64, row: u64, col: u16) -> Self {
		assert!(row < (1 << 48));
		Self {
			value,
			row_lo: row as _,
			row_hi: (row >> 16) as _,
			col,
		}
	}
	fn row(self) -> u64 {
		u64::from(self.row_lo) | u64::from(self.row_hi) << 16
	}
	fn full_value(self) -> BigUint {
		let row: u64 = self.row();
		let col: u64 = self.col.into();
		let mut value = BigUint::from(1u8);
		for r in row - col + 1..=row {
			value *= BigUint::from(r);
		}
		for r in 1..=col {
			value /= BigUint::from(r);
		}
		value
	}
}

fn find_duplicates_in(entries: &mut [PascalEntry]) {
	entries.sort_by_key(|x| x.value);
	for e in entries.chunk_by(|a, b| a.value == b.value) {
		if e.len() == 1 {
			continue;
		}
		for i in 0..e.len() {
			for j in i + 1..e.len() {
				if e[i].full_value() != e[j].full_value() {
					continue;
				}
				println!(
					"({} choose {}) = {} = ({} choose {})",
					e[i].row(),
					e[i].col,
					e[i].full_value(),
					e[j].row(),
					e[j].col
				);
			}
		}
	}
}

fn search_entry_limit(power_of_10: usize) {
	let mut pascal_row = [UInt::from(0u8); 500];
	let mut range = pascal_row.len();
	println!(
		"searching up to 10{}",
		superscript(&format!("{power_of_10}"))
	);
	let limit = UInt::from(10u8).pow(power_of_10 as u32);
	let mut entries: Vec<PascalEntry> = vec![];
	pascal_row[0] = UInt::from(1u8);
	for row in 1u64.. {
		for col in (1..range).rev() {
			pascal_row[col] += pascal_row[col - 1];
			if col > 4 && col as u64 <= row / 2 {
				if is_choose_r(pascal_row[col], 2)
					|| is_choose_r(pascal_row[col], 3)
					|| is_choose_r(pascal_row[col], 4)
				{
					println!("FOUND DUPLICATE {}", pascal_row[col]);
				}
				entries.push(PascalEntry::new(
					pascal_row[col].digits()[0],
					row,
					col.try_into().expect("needs redesign: col > 65535"),
				));
			}
			if pascal_row[col] > limit {
				range = col;
				if col < 10 {
					println!("n choose {col} exceeds limit at row {row}");
				}
			}
		}
		if range <= 5 {
			break;
		}
	}
	println!(
		"memory needed = {}MiB",
		(entries.len() * size_of::<PascalEntry>()) >> 20
	);
	find_duplicates_in(&mut entries);
}

fn search_row_limit(row_limit: u32) {
	println!("Searching up to row {row_limit}");
	let row_limit = u64::from(row_limit);
	let mut pascal_row = vec![0u64; row_limit as usize / 2 + 1];
	pascal_row[0] = 1;
	let mut entries: Vec<PascalEntry> = vec![];
	for row in 0..row_limit {
		if row > 0 && row % 2 == 0 {
			pascal_row[row as usize / 2] = pascal_row[row as usize / 2 - 1].wrapping_mul(2);
			entries.push(PascalEntry::new(
				pascal_row[row as usize / 2],
				row,
				(row / 2) as u16,
			));
		}
		for col in (2..=(std::cmp::max(3, row) - 1) / 2).rev() {
			pascal_row[col as usize] =
				pascal_row[col as usize].wrapping_add(pascal_row[col as usize - 1]);
			entries.push(PascalEntry::new(pascal_row[col as usize], row, col as u16));
		}
		pascal_row[1] = row;
	}
	println!(
		"memory needed = {}MiB",
		(entries.len() * size_of::<PascalEntry>()) >> 20
	);
	find_duplicates_in(&mut entries);
}

fn search_col_limit(col_limit: u16) {
	let mut pascal_row = vec![UInt::from(1u8), UInt::from(4u8), UInt::from(6u8)];
	let mut range = usize::MAX;
	let mut cutoff = UInt::MAX >> (2 * col_limit + 1);
	for i in 2..=col_limit {
		cutoff /= UInt::from(i);
	}
	println!("entry cutoff > 10^{}", cutoff.ilog10());
	for row in 5usize..usize::MAX {
		if row % (1 << 15) == 0 {
			println!("row = {row}");
		}
		for col in (2..min(row / 2 + 1, range)).rev() {
			if 2 * col == row {
				pascal_row.push(pascal_row[row / 2 - 1] * UInt::from(2u8));
			} else if col == 2 {
				pascal_row[col] += UInt::from(row - 1);
			} else {
				let prev = pascal_row[col - 1];
				pascal_row[col] += prev;
			}
			if pascal_row[col] > cutoff {
				// getting dicey
				println!("abandoning column {col}");
				range = col;
				continue;
			}
			if col <= 4 {
				// we already know the solutions with both columns ≤ 4
				continue;
			}
			if is_choose2(pascal_row[col]) {
				println!("({row} choose {col}) = (something choose 2)");
			}
			for col2 in 3..min(col, col_limit as usize + 1) {
				if let Some(x) = find_choose_r(pascal_row[col], col2 as u32) {
					println!("({row} choose {col}) = ({x} choose {col2})");
				}
			}
		}
		if range <= 2 {
			println!("can't go any further without increasing size of UInt");
			return;
		}
	}
}

fn main() -> ExitCode {
	let args: Vec<OsString> = std::env::args_os().collect();
	if args.len() < 2 {
		eprintln!(" Usage: pascal entry-limit <p>  — search all entries up to 10^p");
		eprintln!("        pascal row-limit <n>    — search all entries up to the nth row");
		return ExitCode::FAILURE;
	}
	match args[1].as_os_str().to_str() {
		Some("entry-limit") => {
			let power_of_10: Option<usize> = match args.get(2) {
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
			search_entry_limit(power_of_10);
		}
		Some("row-limit") => {
			let row_limit: Option<u32> = match args.get(2) {
				Some(s) => s.clone().into_string().ok().and_then(|x| x.parse().ok()),
				None => Some(1000),
			};
			let Some(row_limit) = row_limit else {
				eprintln!("Argument must be a nonnegative integer");
				return ExitCode::FAILURE;
			};
			if row_limit > 0xffff * 2 {
				eprintln!("row limit too large (need to change PascalEntry type)");
				return ExitCode::FAILURE;
			}
			search_row_limit(row_limit);
		}
		Some("col-limit") => {
			let col_limit: Option<u16> = match args.get(2) {
				Some(s) => s.clone().into_string().ok().and_then(|x| x.parse().ok()),
				None => Some(30),
			};
			let Some(col_limit) = col_limit else {
				eprintln!("column limit must be a nonnegative integer < 65536");
				return ExitCode::FAILURE;
			};
			search_col_limit(col_limit);
		}
		_ => {
			eprintln!("Bad command: {:?}", args[1]);
		}
	}
	ExitCode::SUCCESS
}
