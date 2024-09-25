pub fn tic_tac_toe(table: Vec<Vec<&str>>) -> String {
    if horizontal("X", &table) {
        return String::from("player X won");
    }

    if vertical("O", &table) {
        return String::from("player O won");
    }

    if diagonals("X", &table) {
        return String::from("player X won");
    }

    String::new()
}

pub fn diagonals(player: &str, table: &Vec<Vec<&str>>) -> bool {
}

pub fn horizontal(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for row in table {
        if row.iter().all(|&cell| cell == player) {
            return true;
        }
    }

    false
}

pub fn vertical(player: &str, table: &Vec<Vec<&str>>) -> bool {
    for col in 0..table.len() {
        if table.iter().all(|row| row[col] == player) {
            return true;
        }
    }

    false
}
