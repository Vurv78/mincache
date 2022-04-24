# ``mincache``
Minimal crate to cache return values of your function returns.

Currently only supports a timer / cooldown cache.

## Example
```rs
pub use mincache::timed;

// "fmt" is what you'd put after core::time::Duration::from_<>. E.g. "secs", "millis", "nanos", etc.
#[timed(t = 5000, fmt = "millis"/*, reference = true*/)]
pub fn func(x: i32) -> std::time::Instant {
	println!("This will print once {x}");

	// Reference non-cached self.
	// inner_func(x);

	// This value will be cloned each time this is called before the cooldown is over
	// If you want to instead pass a reference, look into mincache/tests/ref.rs
	// (just add ``reference = true`` to the attribute params)
	std::time::Instant::now()
}

fn main() {
	// Will call the function a single time.
	for _ in 0..1000000 {
		let _xyz = func(55);
	}
}
```