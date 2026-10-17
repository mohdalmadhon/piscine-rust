pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    for column in table {
        let mut counter: i32 = 0;
        let mut previous: char = ' ';

        for (i, spot) in column.iter().enumerate() {
            if i == 0 {
                counter += 1;
                previous = *spot;
                continue;
            }
            if *spot == previous && *spot != ' ' {
                counter += 1;
                previous = *spot;
                if counter == 3 {
                    return format!("player {} won", previous);
                }
            } else {
                break;
            }
        }
    }

    for i in 0..3 {
        let mut counter: i32 = 0;
        let mut previous: char = ' ';

        for j in 0..3 {
            let spot = table[j][i];

            if j == 0 {
                counter += 1;
                previous = spot;
                continue;
            }
            if spot == previous && spot != ' ' {
                counter += 1;
                previous = spot;
                if counter == 3 {
                    return format!("player {} won", previous);
                }
            } else {
                break;
            }
        }
    }

    for i in 0..3 {
        if table[i][i] != ' ' && table[i][i] == table[0][0] {
            if i == 2 {
                return format!("player {} won", table[0][0]);
            }
        } else {
            break;
        }
    }

    for i in 0..3 {
        if table[i][2 - i] != ' ' && table[i][2 - i] == table[0][2] {
            if i == 2 {
                return format!("player {} won", table[0][2]);
            }
        } else {
            break;
        }
    }

    format!("tie")
}
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
    for player in ['O