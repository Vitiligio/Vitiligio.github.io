use std::io;

/*

fn main() {

    let mut tateti = Tateti::new();

    let result = gameloop(&mut tateti);

    format!("Game ended with result: {}", result);

}

*/

fn main() {
    let mut super_tateti = SuperTateti::new();

    super_tateti.print();
}

pub struct SuperTateti {
    map: Vec<Vec<Tateti>>,
}

impl SuperTateti {
    pub fn new() -> SuperTateti {
        SuperTateti {
            map: vec![
                vec![Tateti::new(), Tateti::new(), Tateti::new()],
                vec![Tateti::new(), Tateti::new(), Tateti::new()],
                vec![Tateti::new(), Tateti::new(), Tateti::new()],
            ],
        }
    }

    pub fn print(&self) {
        for line in self.map.iter() {
            let mut i = 0;

            for tateti in line {
                tateti.print_line(i);

                i += 1;
            }

            println!(" ");
        }
    }
}

pub struct Tateti {
    map: Vec<Vec<char>>,

    winner: String,
}

impl Tateti {
    pub fn new() -> Tateti {
        Tateti {
            map: vec![
                vec![' ', ' ', ' '],
                vec![' ', ' ', ' '],
                vec![' ', ' ', ' '],
            ],

            winner: String::from("Tied"),
        }
    }

    pub fn insert(&mut self, row: usize, column: usize, symbol_to_draw: char) {
        if self.map[row][column] == ' ' {
            self.map[row][column] = symbol_to_draw;
        } else {
            self.ask_imput(symbol_to_draw);
        }
    }

    pub fn print(&self) {
        for line in self.map.iter() {
            println!("{:?}", line);
        }
    }

    pub fn print_line(&self, line: usize) {
        if line < self.map.len() {
            print!("| {:?} |", self.map[line]);
        }
    }

    pub fn manage_input(&mut self, input: &str, symbol_to_draw: char) {
        const UP_LEFT: &str = "ul";

        const UP_CENTER: &str = "uc";

        const UP_RIGHT: &str = "ur";

        const CENTER_LEFT: &str = "cl";

        const CENTER_CENTER: &str = "cc";

        const CENTER_RIGHT: &str = "cr";

        const DOWN_LEFT: &str = "dl";

        const DOWN_CENTER: &str = "dc";

        const DOWN_RIGHT: &str = "dr";

        match input {
            UP_LEFT => {
                self.insert(0, 0, symbol_to_draw);
            }

            UP_CENTER => {
                self.insert(0, 1, symbol_to_draw);
            }

            UP_RIGHT => {
                self.insert(0, 2, symbol_to_draw);
            }

            CENTER_LEFT => {
                self.insert(1, 0, symbol_to_draw);
            }

            CENTER_CENTER => {
                self.insert(1, 1, symbol_to_draw);
            }

            CENTER_RIGHT => {
                self.insert(1, 2, symbol_to_draw);
            }

            DOWN_LEFT => {
                self.insert(2, 0, symbol_to_draw);
            }

            DOWN_CENTER => {
                self.insert(2, 1, symbol_to_draw);
            }

            DOWN_RIGHT => {
                self.insert(2, 2, symbol_to_draw);
            }

            _ => self.ask_imput(symbol_to_draw),
        }
    }

    pub fn ask_imput(&mut self, symbol_to_draw: char) {
        println!("Turn to play: {}\nWrite the direction: ", symbol_to_draw);

        let mut input_string = String::new();

        if let Ok(_result) = io::stdin().read_line(&mut input_string) {
            self.manage_input(&input_string.trim(), symbol_to_draw);
        }
    }

    pub fn is_full(&self) -> bool {
        let mut result = true;

        for vec in self.map.iter() {
            for value in vec {
                if value == &' ' {
                    result = false;
                }
            }
        }

        result
    }

    pub fn is_game_over(&mut self) -> bool {
        (self.is_full() || self.we_have_winner())
    }

    pub fn we_have_winner(&mut self) -> bool {
        let row_cond = self.check_rows();

        let col_cond = self.check_columns();

        let diag_cond = self.check_diags();

        if let Some(char) = row_cond {
            self.winner = format!("The winner is {}\n", char);
        }

        if let Some(char) = col_cond {
            self.winner = format!("The winner is {}\n", char);
        }

        if let Some(char) = diag_cond {
            self.winner = format!("The winner is {}\n", char);
        }

        row_cond.is_some() || col_cond.is_some() || diag_cond.is_some()
    }

    fn check_rows(&self) -> Option<char> {
        let mut winner = None;

        for vec in self.map.iter() {
            if all_vec_items_are_equal(&vec) && vec[0] != ' ' {
                winner = Some(vec[0]);
            }
        }

        winner
    }

    fn check_columns(&self) -> Option<char> {
        let mut winner = None;

        let map = self.transpose_map();

        for vec in map {
            if all_vec_items_are_equal(&vec) && vec[0] != ' ' {
                winner = Some(vec[0]);
            }
        }

        winner
    }

    fn transpose_map(&self) -> Vec<Vec<char>> {
        let mut transposed = vec![
            vec![' ', ' ', ' '],
            vec![' ', ' ', ' '],
            vec![' ', ' ', ' '],
        ];

        for x in 0..self.map.len() {
            for y in 0..self.map.len() {
                transposed[y][x] = self.map[x][y];
            }
        }

        transposed
    }

    fn get_diagonals(&self) -> Vec<Vec<char>> {
        let mut diag = vec![vec![' ', ' ', ' '], vec![' ', ' ', ' ']];

        for y in 0..self.map.len() {
            diag[0][y] = self.map[y][y];

            diag[1][y] = self.map[self.map.len() - 1 - y][self.map.len() - 1 - y];
        }

        diag
    }

    fn check_diags(&self) -> Option<char> {
        let mut winner = None;

        let map = self.get_diagonals();

        for vec in map {
            if all_vec_items_are_equal(&vec) && vec[0] != ' ' {
                winner = Some(vec[0]);
            }
        }

        winner
    }

    pub fn winner(&self) -> String {
        format!("X")
    }
}

pub fn all_vec_items_are_equal(vec: &Vec<char>) -> bool {
    let first_item = vec[0];

    let mut are_equal = true;

    for item in vec {
        if &first_item != item {
            are_equal = false;
        }
    }

    are_equal
}

pub fn gameloop(tateti: &mut Tateti) -> String {
    let mut end_condition = false;

    while !end_condition {
        tateti.ask_imput('O');

        tateti.print();

        if tateti.is_game_over() {
            println!("Game Over\n");

            println!("{}", tateti.winner);

            break;
        }

        tateti.ask_imput('X');

        tateti.print();

        if tateti.is_game_over() {
            println!("Game Over\n");

            println!("{}", tateti.winner);

            break;
        }

        end_condition = tateti.is_game_over();
    }

    format!("Game ended with result: {}", tateti.winner())
}
