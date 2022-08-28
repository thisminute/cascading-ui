use std::sync::atomic::{AtomicUsize, Ordering};

static CLASS_COUNTER: AtomicUsize = AtomicUsize::new(0);
static MUTABLE_COUNTER: AtomicUsize = AtomicUsize::new(0);
static SYMBOLS_1: &[char] = &[
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
	't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
	'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_',
];
static SYMBOLS_2: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-'];

pub fn generate_class_id() -> String {
	let mut id = String::from("");
	let mut n = CLASS_COUNTER.load(Ordering::Relaxed);
	CLASS_COUNTER.swap(n + 1, Ordering::Relaxed);

	loop {
		let symbol_pool = if id.is_empty() {
			SYMBOLS_1.len()
		} else {
			SYMBOLS_1.len() + SYMBOLS_2.len()
		};
		let digit = n % symbol_pool;
		id.push(if digit < SYMBOLS_1.len() {
			SYMBOLS_1[digit]
		} else {
			SYMBOLS_2[digit]
		});
		n /= symbol_pool;
		if n == 0 {
			break;
		}
	}
	id
}

pub fn generate_mutable_id() -> usize {
	let n = MUTABLE_COUNTER.load(Ordering::Relaxed);
	MUTABLE_COUNTER.swap(n + 1, Ordering::Relaxed);
	n
}
