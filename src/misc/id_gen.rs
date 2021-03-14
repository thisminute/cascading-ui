use std::sync::atomic::{AtomicUsize, Ordering};

// static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
static CLASS_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SYMBOLS_1: &[char] = &[
	'_', 'a',
	'b',
	// 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
	// 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
	// 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
static SYMBOLS_2: &[char] = &[
	'0', '1', '2', //, '3', '4', '5', '6', '7', '8', '9', '-'
];

#[derive(Debug)]
pub enum IdCategory {
	// Id,
	Class,
}

pub fn id_gen(category: IdCategory) -> String {
	let counter = match category {
		// IdCategory::Id => &ID_COUNTER,
		IdCategory::Class => &CLASS_COUNTER,
	};

	let mut id = String::from("");
	let mut n = counter.load(Ordering::Relaxed);
	counter.swap(n + 1, Ordering::Relaxed);

	while {
		let symbol_pool = if id.len() == 0 {
			SYMBOLS_1.len()
		} else {
			SYMBOLS_1.len() + SYMBOLS_2.len()
		};
		let digit = n % symbol_pool;
		if digit < SYMBOLS_1.len() {
			id.push(SYMBOLS_1[digit]);
		} else {
			id.push(SYMBOLS_2[digit]);
		}
		n /= symbol_pool;
		n > 0
	} {}
	eprintln!("{:?} generated: '{}'", category, id);
	id
}
