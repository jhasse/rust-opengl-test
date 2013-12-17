use std::path::Path;
use std::io::fs::readlink;

struct Paths {
	prefix: Path
}

impl Paths {
	pub fn new() -> Paths {
		let target = readlink(&Path::new("/proc/self/exe")).unwrap();
		let prefix = target.join(Path::new("../.."));
		debug!("Prefix path: {}", prefix.as_str().unwrap());
		return Paths{ prefix: prefix };
	}
}
