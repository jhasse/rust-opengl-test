use std::old_path::Path;
use std::env;

pub struct Paths {
	pub prefix: Path
}

impl Paths {
	pub fn new() -> Paths {
		let mut tmp = match env::current_exe() {
			Ok(p) => p,
			Err(e) => panic!("Can't find exe path: {}", e)
		};
		tmp.pop();
		tmp.pop();
		let prefix = tmp;
		println!("Prefix path: {}", prefix.as_str().unwrap());
		return Paths{ prefix: prefix };
	}
}
