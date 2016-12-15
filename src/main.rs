use std::io::Read;
use std::fs::File;

mod lexer;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // `s` contains the contents of "foo.txt"
    Ok(s)
}

fn main() {
    match read_file("test.bas") {
        Ok(s) => {
            for x in s.lines() {
                println!("Lines: {}", x);
            }
        }
        Err(err) => println!("Getting file contents failed with error: {}", err),
    };
}
