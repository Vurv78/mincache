pub use mincache::timed;

#[test]
fn basic() {
	#[timed(t = 10, fmt = "millis")]
	pub fn xyz(_x: i32) -> std::time::Instant {
		println!("Once");
		std::time::Instant::now()
	}

	for _ in 0..1000000 {
		let _xyz = xyz(55);
	}
}

#[test]
fn inner() {
	// Probably shouldn't use a timecache for a fibbonacci function, but who knows?
	#[timed(t = 5, fmt = "secs")]
	pub fn bad_fib(x: u32) -> u32 {
		if x < 2 {
			x
		} else {
			// Reference non-timed function.
			inner_bad_fib(x - 1) + inner_bad_fib(x - 2)
		}
	}

	for _ in 0..10 {
		let _xyz = bad_fib(5);
	}
}