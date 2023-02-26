mod day01;
mod day02;
mod day03;
mod utils;

fn main() {
    let text_input = utils::read_file("inputs/01a.txt".to_string());
    println!("{}", day01::day01a(text_input.clone()));
    println!("{}", day01::day01b(text_input.clone()));
    let text_input_02 = utils::read_file("inputs/02a.txt".to_string());
    println!("{}", day02::day02a(text_input_02.clone()));
    println!("{}", day02::day02b(text_input_02.clone()));

    let text_input_03 = utils::read_file("inputs/03a.txt".to_string());
    println!("{}", day03::day03a(text_input_03.clone()));
    println!("{}", day03::day03b(text_input_03.clone()));
}
