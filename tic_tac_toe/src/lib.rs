pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let mut counter: i32 = 0;
    for i in 0..3 {
        if table[i][i] == player {
            counter += 1;
        }
    }
    if counter == 3 {
        return true;
    }

    counter = 0;
    for i in 0..3 {
        if table[i][2 - i] == player {
            counter += 1;
        }
    }

    counter == 3
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table {
        let mut counter: i32 = 0;

        for spot in row {
            if spot == player {
                counter += 1;
            }
        }

        if counter == 3 {
            return true;
        }
    }

    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for column in 0..3 {
        let mut counter: i32 = 0;

        for row in 0..3 {
            if table[row][column] == player {
                counter += 1;
            }
        }

        if counter == 3 {
            return true;
        }
    }

    false
}

pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    for player in ['O', 'X'] {
        if diagonals(player, table)
            || horizontal(player, table)
            || vertical(player, table)
        {
            return format!("player {} won", player);
        }
    }

    String::from("tie")
}

fn main() {
    println!(
        "{}",
        tic_tac_toe([['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']])
    );
    // tie
    println!(
        "{}",
        tic_tac_toe([['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']])
    );
    // player O won

    let diag = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];

    println!("{}", tic_tac_toe(diag));
    // player X won
}