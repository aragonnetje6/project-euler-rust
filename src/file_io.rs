use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file(filepath: &str) -> String {
    let path = Path::new(filepath);
    let display = path.display();
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {display}: {why}"),
        Ok(file) => file,
    };

    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) { panic!("couldn't read {display}: {why}") }
    s
}
