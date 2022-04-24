use mincache::timed;

#[test]
fn reference() {
	#[timed(t = 5, fmt = "secs", reference = true)]
	pub fn ref_function(x: u32) -> &'static u32 {
		// Heavy computation...
		Box::leak( Box::new(x + 50) )
	}

	for _ in 0..10 {
		let _xyz = ref_function(55);
	}
}