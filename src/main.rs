mod manage_input;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
extern crate regex;

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

    // December, 4
    let ans_day4: (usize, usize) = day04::answers_day4();
    println!("The answer for day 4 part 1 is: {}", ans_day4.0);
    println!("The answer for day 4 part 2 is: {}", ans_day4.1);

    // December, 5
    let ans_day5 = day05::answers_day5();
    println!("The answer for day 5 part 1 is: {}", ans_day5.0);
    println!("The answer for day 5 part 2 is: {}", ans_day5.1);

    // December, 6
    let ans_day6 = day06::answers_day6();
    println!("The answer for day 6 part 1 is: {}", ans_day6.0);
    println!("The answer for day 6 part 2 is: {}", ans_day6.1);

    // December, 7
    //let ans_day7 = day07::answers_day7();
    //println!("The answer for day 7 part 1 is: {}", ans_day7.0);
    //println!("The answer for day 7 part 2 is: {}", ans_day7.1);

    // December, 8
    let ans_day8 = day08::answers_day8();
    println!("The answer for day 8 part 1 is: {}", ans_day8.0);
    println!("The answer for day 8 part 2 is: {}", ans_day8.1);

    // December, 9
    let ans_day9 = day09::answers_day9();
    println!("The answer for day 9 part 1 is: {}", ans_day9.0);
    println!("The answer for day 9 part 2 is: {}", ans_day9.1);

    // December, 10
    let ans_day10 = day10::answers_day10();
    println!("The answer for day 10 part 1 is: {}", ans_day10.0);
    println!("The answer for day 10 part 2 is: {}", ans_day10.1);

    // December, 11
    let ans_day11 = day11::answers_day11();
    println!("The answer for day 11 part 1 is: {}", ans_day11.0);
    //println!("The answer for day 11 part 2 is: {}", ans_day11.1);
}
