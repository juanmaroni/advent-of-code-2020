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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
extern crate regex;
extern crate peg;

fn main() {
    // December, 1
    let ans_day1 = day01::answers_day1();
    println!(
        "The answer for day 1 part 1 is: {}\nThe answer for day 1 part 2 is: {}\n",
        ans_day1.0, ans_day1.1
    );

    // December, 2
    let ans_day2 = day02::answers_day2();
    println!(
        "The answer for day 2 part 1 is: {}\nThe answer for day 2 part 2 is: {}\n",
        ans_day2.0, ans_day2.1
    );

    // December, 3
    let ans_day3 = day03::answers_day3();
    println!(
        "The answer for day 3 part 1 is: {}\nThe answer for day 3 part 2 is: {}\n",
        ans_day3.0, ans_day3.1
    );

    // December, 4
    let ans_day4 = day04::answers_day4();
    println!(
        "The answer for day 4 part 1 is: {}\nThe answer for day 4 part 2 is: {}\n",
        ans_day4.0, ans_day4.1
    );

    // December, 5
    let ans_day5 = day05::answers_day5();
    println!(
        "The answer for day 5 part 1 is: {}\nThe answer for day 5 part 2 is: {}\n",
        ans_day5.0, ans_day5.1
    );

    // December, 6
    let ans_day6 = day06::answers_day6();
    println!(
        "The answer for day 6 part 1 is: {}\nThe answer for day 6 part 2 is: {}\n",
        ans_day6.0, ans_day6.1
    );

    // December, 7
    let ans_day7 = day07::answers_day7();
    println!(
        "The answer for day 7 part 1 is: {}\nThe answer for day 7 part 2 is: {}\n",
        ans_day7.0, ans_day7.1
    );

    // December, 8
    let ans_day8 = day08::answers_day8();
    println!(
        "The answer for day 8 part 1 is: {}\nThe answer for day 8 part 2 is: {}\n",
        ans_day8.0, ans_day8.1
    );

    // December, 9
    let ans_day9 = day09::answers_day9();
    println!(
        "The answer for day 9 part 1 is: {}\nThe answer for day 9 part 2 is: {}\n",
        ans_day9.0, ans_day9.1
    );

    // December, 10
    let ans_day10 = day10::answers_day10();
    println!(
        "The answer for day 10 part 1 is: {}\nThe answer for day 10 part 2 is: {}\n",
        ans_day10.0, ans_day10.1
    );
/*
    // December, 11
    let ans_day11 = day11::answers_day11();
    println!("The answer for day 11 part 1 is: {}", ans_day11.0);
    //println!("The answer for day 11 part 2 is: {}", ans_day11.1);

    // December, 12
    let ans_day12 = day12::answers_day12();
    println!("The answer for day 12 part 1 is: {}", ans_day12.0);
    println!("The answer for day 12 part 2 is: {}", ans_day12.1);

    // December, 13
    let ans_day13 = day13::answers_day13();
    println!("The answer for day 13 part 1 is: {}", ans_day13.0);
    //println!("The answer for day 13 part 2 is: {}", ans_day13.1);
    

    // December, 14
    let ans_day14 = day14::answers_day14();
    println!("The answer for day 14 part 1 is: {}", ans_day14.0);
    //println!("The answer for day 14 part 2 is: {}", ans_day14.1);

    // December, 15
    let ans_day15 = day15::answers_day15();
    println!("The answer for day 15 part 1 is: {}", ans_day15.0);
    println!("The answer for day 15 part 2 is: {}", ans_day15.1);

    // December, 16
    let ans_day16 = day16::answers_day16();
    println!("The answer for day 16 part 1 is: {}", ans_day16.0);
    //println!("The answer for day 16 part 2 is: {}", ans_day16.1);
    

    // December, 17
    let ans_day17 = day17::answers_day17();
    println!("The answer for day 17 part 1 is: {}", ans_day17.0);
    //println!("The answer for day 17 part 2 is: {}", ans_day17.1);

    // December, 18
    let ans_day18 = day18::answers_day18();
    println!("The answer for day 18 part 1 is: {}", ans_day18.0);
    println!("The answer for day 18 part 2 is: {}", ans_day18.1);

    // December, 19
    let ans_day19 = day19::answers_day19();
    //println!("The answer for day 19 part 1 is: {}", ans_day19.0);
    //println!("The answer for day 19 part 2 is: {}", ans_day19.1);

    // December, 20
    let ans_day20 = day20::answers_day20();
    //println!("The answer for day 20 part 1 is: {}", ans_day20.0);
    //println!("The answer for day 20 part 2 is: {}", ans_day20.1);

    // December, 21
    let ans_day21 = day21::answers_day21();
    //println!("The answer for day 21 part 1 is: {}", ans_day21.0);
    //println!("The answer for day 21 part 2 is: {}", ans_day21.1);

    // December, 22
    let ans_day22 = day22::answers_day22();
    println!("The answer for day 22 part 1 is: {}", ans_day22.0);
    //println!("The answer for day 22 part 2 is: {}", ans_day22.1);

    // December, 23
    let ans_day23 = day23::answers_day23();
    println!("The answer for day 23 part 1 is: {}", ans_day23.0);
    //println!("The answer for day 23 part 2 is: {}", ans_day23.1);

    // December, 24
    let ans_day24 = day24::answers_day24();
    //println!("The answer for day 24 part 1 is: {}", ans_day24.0);
    //println!("The answer for day 24 part 2 is: {}", ans_day24.1);

    // December, 25
    let ans_day25 = day25::answers_day25();
    println!("The answer for day 25 part 1 is: {}", ans_day25);
    println!("The answer for day 25 part 2 is: MERRY CHRISTMAS!");
*/   
}
