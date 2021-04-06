use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(1);
static SYMBOLS_1: &[char] = &[
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
	't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
	'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_',
];
static SYMBOLS_2: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-'];

pub fn id_gen() -> String {
	let mut id = String::from("");
	let mut n = COUNTER.load(Ordering::Relaxed);
	COUNTER.swap(n + 1, Ordering::Relaxed);

	while n > 0 {
		let symbol_pool = if id.len() == 0 {
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
	}
	id
}
