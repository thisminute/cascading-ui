use std::sync::atomic::{AtomicUsize, Ordering};

static _COUNTER: AtomicUsize = AtomicUsize::new(0);
static _SYMBOLS: &[char] = &['a', 'b', 'c', 'A', 'B', 'C'];

pub fn _id_gen() -> String {
	let mut id = "_".to_string();
	let mut n = _COUNTER.load(Ordering::Relaxed);
	_COUNTER.swap(n + 1, Ordering::Relaxed);
	while n > 0 {
		let digit = n % _SYMBOLS.len();
		id.push(_SYMBOLS[digit]);
		n /= _SYMBOLS.len();
	}
	id
}
