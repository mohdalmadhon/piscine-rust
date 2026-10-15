
use std::io;

const RIDDLE: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
const ANSWER: &str = "The letter e";
fn main() {
    let mut buf = String::new();
    let mut n: u32 = 0;

    loop {
        print_riddle();

        buf.clear();

        io::stdin().read_line(&mut buf)
        .expect("please enter an input");

        n += 1;

        if buf.trim().eq_ignore_ascii_case(ANSWER) {
            println!("Number of trials: {}", n);
            break;
        }

        continue;
    }
}

fn print_riddle() -> () {
    println!("{RIDDLE}");
}