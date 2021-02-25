use rand::Rng;
use std::io;

const NAME: &str = "Rusted Life";
const BOARD_SIZE: usize = 24;
const COLUMN_IDS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'X', 'Y', 'Z',
];
const ROW_IDS: &[char] = &[
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '#', '+', '-', '*', '~', '!', '%', '&', '"', '§',
    '/', '(', ')', '=', '?', '°'
];
const CURSOR: &[char; 2] = &['[', ']'];

enum CellState {
    Alive,
    Dead,
}
impl CellState {
    fn to_char(&self) -> char {
        match self {
            CellState::Alive => '#',
            CellState::Dead => '.',
        }
    }
}

enum FinderResult {
    NotFound,
    Index(usize),
}

fn findStringInCharArray(array: &[char], c: &str) -> FinderResult {
    let mut spot: FinderResult = FinderResult::NotFound;
    for (index, &item) in array.iter().enumerate() {
        if item.to_string() == c.to_string() {
            spot = FinderResult::Index(index);
        }
    }
    return spot;
}

fn main() {
    let mut board = [[&CellState::Dead; BOARD_SIZE]; BOARD_SIZE];
    loop {
        let result: u8 = startpage(&mut board);
        if result == 1 {
            // START GAME
            let r: u8 = simulator(&mut board);
            if r != 2 {
                // quit
                break;
            } // else reopen editor
        }
    }
}

fn startpage(board: &mut [[&CellState; BOARD_SIZE]; BOARD_SIZE]) -> u8 {
    let mut pointer: [usize; 2] = [0; 2];

    let result: u8 = loop {
        // clear screen
        print!("{}[2J", 27 as char);

        let mut ui = String::from("\n     ");
        ui.push_str(NAME);
        ui.push_str("\n        EDITOR\n");
        ui.push_str("\n\n\n");

        // COLUMN NAMES
        ui.push_str(&String::from("      "));
        for c in (0..BOARD_SIZE) {
            ui = ui + &String::from("   ") + &COLUMN_IDS[c].to_string()
        }
        ui.push_str(&String::from("\n\n"));

        // ROWS
        for (r, &row) in board.iter().enumerate() {
            let mut line = String::from("\n   ");
            // ROW NAMES
            line = line + &ROW_IDS[r].to_string() + &"   ".to_string();

            // ROW ITEMS
            for (i, &item) in row.iter().enumerate() {
                let end_tag = String::from(if pointer[0] == r && pointer[1] == i {
                    "]"
                } else {
                    " "
                });
                let start_tag = String::from(if pointer[0] == r && pointer[1] == i {
                    " ["
                } else {
                    "  "
                });

                line = line + &start_tag + &item.to_char().to_string() + &end_tag;
            }
            line.push_str(&String::from("   \n"));
            ui.push_str(&line);
        }
        ui.push_str(&String::from("\n\n"));
        ui.push_str("Enter Command [e.g. q, s, r, c, m, A, K, C1, B6]");

        // PRINT UI
        println!("{}", ui);

        // GET USER INPUT
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        command = command.trim().to_string();

        if command.len() == 2 {
            // NAVIGATE
            let col = &command[..1];
            let row = &command[1..];

            let row_index = match findStringInCharArray(&COLUMN_IDS, &col) {
                FinderResult::NotFound => continue,
                FinderResult::Index(i) => i,
            };
            let column_index = match findStringInCharArray(&ROW_IDS, &row) {
                FinderResult::NotFound => continue,
                FinderResult::Index(i) => i,
            };

            // TODO Pointer x & y seem to be swapped
            pointer = [column_index, row_index];
        } else if command == "q" {
            // QUIT
            break 0;
        } else if command == "s" {
            // START SIMULATION
            break 1;
        } else if command == "r" {
            // RANDOM SETUP
            let mut new_board = [[&CellState::Dead; BOARD_SIZE]; BOARD_SIZE];
            for (row_index, &row) in board.iter().enumerate() {
                for (column_index, &column) in row.iter().enumerate() {
                    let r = rand::thread_rng().gen_range(0, 3);
                    if r == 0 {
                        new_board[row_index][column_index] = &CellState::Alive;
                    } else {
                        new_board[row_index][column_index] = &CellState::Dead;
                    }
                }
            }
            *board = new_board;
        }else if command == "c" {
            let mut new_board = [[&CellState::Dead; BOARD_SIZE]; BOARD_SIZE];
            for (row_index, &row) in board.iter().enumerate() {
                for (column_index, &column) in row.iter().enumerate() {
                        new_board[row_index][column_index] = &CellState::Dead;
                }
            }
            *board = new_board;
        } else if command == "m" {
             let mut new_board = [[&CellState::Dead; BOARD_SIZE]; BOARD_SIZE];
            for (row_index, &row) in board.iter().enumerate() {
                for (column_index, &column) in row.iter().enumerate() {
                        new_board[row_index][column_index] = &CellState::Alive;
                }
            }
            *board = new_board;
        } else if command == "A" {
            // MARK POINTER CELL AS ALIVE
            board[pointer[0]][pointer[1]] = &CellState::Alive;
        } else if command == "K" {
            // MARK POINTER CELL AS DEAD
            board[pointer[0]][pointer[1]] = &CellState::Dead;
        } else if command == "R" {
            let r = rand::thread_rng().gen_range(0, 3);
            if r == 0 {
                board[pointer[0]][pointer[1]] = &CellState::Alive;
            } else {
                board[pointer[0]][pointer[1]] = &CellState::Dead;
            }
        } else {
            // UNKNOWN COMAND
        }
    };
    return result;
}

fn simulator(board: &mut [[&CellState; BOARD_SIZE]; BOARD_SIZE]) -> u8 {
    let mut generation = 0;

    let result: u8 = loop {
        // clear screen
        print!("{}[2J", 27 as char);

        let mut ui = String::from("\n     ");
        ui.push_str(NAME);
        ui.push_str("\n        SIMULATOR\n");
        ui.push_str("\n\n\n");
        ui.push_str("Generation: ");
        ui.push_str(&generation.to_string());
        ui.push_str("\n\n");

        // COLUMN NAMES
        ui.push_str(&String::from("      "));
        for c in (0..BOARD_SIZE) {
            ui = ui + &String::from("   ") + &COLUMN_IDS[c].to_string()
        }
        ui.push_str(&String::from("\n\n"));

        // ROWS
        for (r, &row) in board.iter().enumerate() {
            let mut line = String::from("\n   ");
            // ROW NAMES
            line = line + &ROW_IDS[r].to_string() + &"   ".to_string();

            // ROW ITEMS
            for (i, &item) in row.iter().enumerate() {
                let end_tag = " ";
                let start_tag = "  ";

                line = line + &start_tag + &item.to_char().to_string() + &end_tag;
            }
            line.push_str(&String::from("   \n"));
            ui.push_str(&line);
        }
        ui.push_str(&String::from("\n\n"));
        ui.push_str("Enter Command [e.g. q, e, s10, s100, other commands will go to next generation]");

        // PRINT UI
        println!("{}", ui);

        // GET USER INPUT
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        command = command.trim().to_string();

        let mut skip: u8 = 1;

        if command == "q" {
            // QUIT
            break 0;
        } else if command == "e" {
            // OPEN EDITOR
            break 2;
        } else if command.len() >= 2 {
            let cmd = &command[..1];
            let params = &command[1..];
            if cmd == "s" {
                // SKIP
                skip = match params.parse() {
                    Ok(n) => n,
                    Err(_) => 1
                };
            }
        } else {
            // SPECIFY RUNS
            skip = 1;
        };

        let mut new_board = [[&CellState::Dead; BOARD_SIZE]; BOARD_SIZE];
        for run in (0..skip) {
            generation = generation + 1;
            println!("{}", run);
            // GENERATE NEXT GENERATION
            for (row_index, &row) in board.iter().enumerate() {
                for (column_index, &column) in row.iter().enumerate() {
                    let alive_nb: u8 = loop {
                        let mut counter: u8 = 0;

                        //DIRECT
                        if column_index > 0 {
                            // check left nb
                            match board[row_index][column_index - 1] {
                                CellState::Dead => (),
                                CellState::Alive => counter = counter + 1,
                            };
                        }
                        if column_index < BOARD_SIZE - 1 {
                            // check right nb
                            match board[row_index][column_index + 1] {
                                CellState::Dead => (),
                                CellState::Alive => counter = counter + 1,
                            };
                        }
                        if row_index > 0 {
                            // check upper nb
                            match board[row_index - 1][column_index] {
                                CellState::Dead => (),
                                CellState::Alive => counter = counter + 1,
                            };
                        }
                        if row_index < BOARD_SIZE - 1 {
                            // check bottom nb
                            match board[row_index + 1][column_index] {
                                CellState::Dead => (),
                                CellState::Alive => counter = counter + 1,
                            };
                        }

                        // CORNER
                        if column_index > 0 && row_index > 0 {
                            // check top left nb
                            match board[row_index - 1][column_index - 1] {
                                CellState::Dead => (),
                                CellState::Alive => counter = counter + 1,
                            };
                        }
                        if column_index < BOARD_SIZE - 1 && row_index > 0 {
                            // check top right nb
                            match board[row_index - 1][column_index + 1] {
                                CellState::Dead => (),
                                CellState::Alive => counter = counter + 1,
                            };
                        }
                        if column_index > 0 && row_index < BOARD_SIZE - 1 {
                            // check bottom left nb
                            match board[row_index + 1][column_index - 1] {
                                CellState::Dead => (),
                                CellState::Alive => counter = counter + 1,
                            };
                        }
                        if column_index < BOARD_SIZE - 1 && row_index < BOARD_SIZE - 1 {
                            // check bottom right nb
                            match board[row_index + 1][column_index + 1] {
                                CellState::Dead => (),
                                CellState::Alive => counter = counter + 1,
                            };
                        }
                        break counter;
                    };
                    if alive_nb < 2 {
                        // die
                        new_board[row_index][column_index] = &CellState::Dead;
                    } else if alive_nb >= 2 && alive_nb < 4 {
                        // alive
                        match board[row_index][column_index] {
                            CellState::Dead => {
                                if alive_nb == 3 {
                                    new_board[row_index][column_index] = &CellState::Alive;
                                }
                            }
                            CellState::Alive => {
                                new_board[row_index][column_index] = &CellState::Alive;
                            }
                        };
                    } else if alive_nb >= 4 {
                        // die
                        new_board[row_index][column_index] = &CellState::Dead;
                    }
                }
            }
            *board = new_board;
        }
    };
    return result;
}
