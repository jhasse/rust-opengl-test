use std::path::Path;
use std::os::self_exe_path;

pub struct Paths {
	pub prefix: Path
}

impl Paths {
	pub fn new() -> Paths {
		let mut tmp = match self_exe_path() {
			Some(p) => p,
			None => fail!("Can't find exe path")
		};
		tmp.pop();
		let prefix = tmp;
		println!("Prefix path: {}", prefix.as_str().unwrap());
		return Paths{ prefix: prefix };
	}
}
