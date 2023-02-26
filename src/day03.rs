pub fn day03a(puzzle_input: String) -> i32 {
    let mut priority_sum: i32 = 0;
    let pairs = puzzle_input.lines();
    for pair in pairs {
        let sacks_size = pair.len();
        let split_point = sacks_size / 2;
        let sack1 = &pair[..split_point];
        let sack2 = &pair[split_point..];
        priority_sum += get_char_value(find_duplicate(sack1, sack2));
    }
    return priority_sum;
}

pub fn day03b(puzzle_input: String) -> i32 {
    let mut line_state = 2;
    let mut line1 = "";
    let mut line2 = "";
    let mut line3 = "";
    let mut priority_sum = 0;

    let lines = puzzle_input.lines();
    for line in lines {
        line_state = (line_state + 1) % 3;
        if line_state == 0 {
            line1 = line.clone();
        }
        if line_state == 1 {
            line2 = line.clone();
        }
        if line_state == 2 {
            line3 = line.clone();
            let duplicate_sack = duplicate_sack_creator(line1, line2);
            let duplicate = find_duplicate(duplicate_sack.as_str(), line3);
            priority_sum += get_char_value(duplicate)
        }
    }
    return priority_sum;
}

fn duplicate_sack_creator(str_a: &str, str_b: &str) -> String {
    let mut duplicate_sack = String::new();

    for charac in str_a.chars() {
        for charact in str_b.chars() {
            if charact == charac {
                duplicate_sack.push(charac);
            }
        }
    }
    return duplicate_sack;
}

fn find_duplicate(str_a: &str, str_b: &str) -> char {
    let mut matched_character = '0';
    for charac in str_a.chars() {
        for charact in str_b.chars() {
            if charact == charac {
                matched_character = charac;
            }
        }
    }
    return matched_character;
}

fn get_char_value(charac: char) -> i32 {
    match charac {
        'a'..='z' => return charac as i32 - 96,
        'A'..='Z' => return charac as i32 - 64 + 26,
        _ => return 0,
    }
}
