use std::cmp;

pub fn day01a(puzzle_input: String) -> i32 {
    let mut calorie_count = 0;
    let mut calorie_max = 0; //No -ve Calories
    let elves_food = puzzle_input.lines();
    for elf_food in elves_food {
        if !elf_food.is_empty() {
            calorie_count += elf_food
                .trim()
                .parse::<i32>()
                .expect("Was expecting a number");
        } else {
            calorie_max = cmp::max(calorie_count, calorie_max);
            calorie_count = 0;
        }
    }
    return calorie_max;
}
pub fn day01b(puzzle_input: String) -> i32 {
    let mut elves_calories = Vec::new();
    let mut calorie_count = 0;
    let elves_food = puzzle_input.lines();
    for elf_food in elves_food {
        if !elf_food.is_empty() {
            calorie_count += elf_food
                .trim()
                .parse::<i32>()
                .expect("Was expecting a number");
        } else {
            elves_calories.push(calorie_count);
            calorie_count = 0;
        }
    }
    elves_calories.sort();
    elves_calories.reverse();

    return elves_calories[0] + elves_calories[1] + elves_calories[2];
}
