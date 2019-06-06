use ferris_says; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;
    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(out, width, &mut writer).unwrap();
}
