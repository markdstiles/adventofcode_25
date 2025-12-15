pub mod day_6;

use colored::Colorize;

fn main() -> anyhow::Result<()> {
    
    if let Ok(answer) = day_6::do_part1() {
        println!("{} {}", "Part 1 answer:".green(), answer.to_string().blue().bold());
    }

    if let Ok(answer) = day_6::do_part2() {
        println!("{} {}", "Part 2 answer:".green(), answer.to_string().blue().bold());
    }

    Ok(())
}
