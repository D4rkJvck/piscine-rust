pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if horizontal("X", &table) || vertical("X", &table) || diagonals("X", &table) {
        return String::from("player X won");
    }

    if horizontal("O", &table) || vertical("O", &table) || diagonals("O", &table) {
        return String::from("player O won");
    }

    String::from("tie")
}

////////////////////////////////////////////////////////////////////////

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
    if table[1][1] != player {
        return false;
    }

    if table[0][0] == player && table[2][2] == player {
        return true;
    }

    if table[0][2] == player && table[2][0] == player {
        return true;
    }

    false
}

////////////////////////////////////////////////////////////////////////

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for row in table {
        if row.iter().all(|&cell| cell == player) {
            return true;
        }
    }

    false
}

////////////////////////////////////////////////////////////////////////

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for col in 0..table.len() {
        if table.iter().all(|row| row[col] == player) {
            return true;
        }
    }

    false
}
