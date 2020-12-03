mod manage_input;
mod day01;
mod day02;
mod day03;

fn main() {
    // December, 1
    let ans_day1_part1: i32 = day01::answer_day1_part1();
    println!("The answer for day 1 part 1 is: {}", ans_day1_part1);

    let ans_day1_part2: i32 = day01::answer_day1_part2();
    println!("The answer for day 1 part 2 is: {}", ans_day1_part2);

    // December, 2
    let ans_day2: (u16, u16) = day02::answers_day2();
    println!("The answer for day 2 part 1 is: {}", ans_day2.0);
    println!("The answer for day 2 part 2 is: {}", ans_day2.1);

    // December, 3
    let ans_day3: (usize, usize) = day03::answers_day3();
    println!("The answer for day 3 part 1 is: {}", ans_day3.0);
    println!("The answer for day 3 part 2 is: {}", ans_day3.1);
}
