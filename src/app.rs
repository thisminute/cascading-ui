#[macro_use]
mod cwf;

pub fn run() {
	let runtime = cwf::Runtime::new();
	cwf!(runtime
		div {
			text: hello;
		}
		span {
			text: world;
		}
	);
}
