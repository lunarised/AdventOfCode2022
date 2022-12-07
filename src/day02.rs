pub fn day02a(puzzle_input: String) -> i32 {
    let win_score = 6;
    let draw_score = 3;
    let loss_score = 0;

    let rock = 1;
    let paper = 2;
    let scissors = 3;

    let mut score = 0;
    let turns = puzzle_input.lines();
    for turn in turns {
        match turn {
            "A X" => score += rock + draw_score,
            "A Y" => score += paper + win_score,
            "A Z" => score += scissors + loss_score,
            "B X" => score += rock + loss_score,
            "B Y" => score += paper + draw_score,
            "B Z" => score += scissors + win_score,
            "C X" => score += rock + win_score,
            "C Y" => score += paper + loss_score,
            "C Z" => score += scissors + draw_score,
            _ => score += 0,
        }
    }

    return score;
}

pub fn day02b(puzzle_input: String) -> i32 {
    let win_score = 6;
    let draw_score = 3;
    let loss_score = 0;

    let rock = 1;
    let paper = 2;
    let scissors = 3;

    let mut score = 0;
    let turns = puzzle_input.lines();
    for turn in turns {
        match turn {
            "A X" => score += scissors + loss_score,
            "A Y" => score += rock + draw_score,
            "A Z" => score += paper + win_score,
            "B X" => score += rock + loss_score,
            "B Y" => score += paper + draw_score,
            "B Z" => score += scissors + win_score,
            "C X" => score += paper + loss_score,
            "C Y" => score += scissors + draw_score,
            "C Z" => score += rock + win_score,
            _ => score += 0,
        }
    }

    return score;
}
