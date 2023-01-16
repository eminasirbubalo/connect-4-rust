#![allow(unused)]

use ::std::io;
use ::std::str;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::process;
use std::process::exit;

fn main() {
    let (rows, columns) = get_board_size();
    let mut gameboard = [['-'; 20]; 20];

    //START OF THE GAME
    print!("\x1B[2J\x1B[1;1H");
    print_board(rows, columns, gameboard);
    game(rows, columns, &mut gameboard);

    while (true) {
        println!("Do you want to play again? Y/N?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().chars().next().unwrap();
        if (choice == 'Y' || choice == 'y') {
            let (rows, columns) = get_board_size();
            let mut gameboard = [['-'; 20]; 20];
            //START OF THE GAME
            print!("\x1B[2J\x1B[1;1H");
            print_board(rows, columns, gameboard);
            game(rows, columns, &mut gameboard);
        } else if (choice == 'N' || choice == 'n') {
            println!("Thank You for playing, goodbye!");
            break;
        } else {
            println!("Please type either Y/y for yes and N/n for no ! ");
        }
    }
}

fn get_board_size() -> (usize, usize) {
    let mut valid_input = 0;
    let mut rows = 0;
    let mut columns = 0;
    while valid_input != 1 {
        print!("\x1B[2J\x1B[1;1H");
        println!("******CONNECT 4******");
        let mut rows_string = String::new();

        println!("Create custom size of your board! Make sure your board is at least 6x7, but be careful difference between rows and columns must be less then 2!");
        println!("Please enter the number of rows:");
        io::stdin()
            .read_line(&mut rows_string)
            .expect("Failed to read line");
        let rows_num: usize = rows_string
            .trim()
            .parse()
            .expect("Only numbers are expected");

        println!("Please enter the number of columns:");
        let mut columns_string = String::new();
        io::stdin()
            .read_line(&mut columns_string)
            .expect("Failed to read line");
        let columns_num: usize = columns_string
            .trim()
            .parse()
            .expect("Only numbers are expected");

        let mut diff = 0;
        if rows_num > columns_num {
            diff = rows_num - columns_num;
        } else if columns_num > rows_num {
            diff = columns_num - rows_num;
        } else {
            diff = 0;
        }

        if columns_num >= 7 && rows_num >= 6 && diff <= 2 {
            valid_input = 1;
            columns = columns_num;
            rows = rows_num;
            break;
        }
    }
    return (rows, columns);
}

fn game(rows: usize, columns: usize, gameboard: &mut [[char; 20]; 20]) {
    let mut player = 1;
    let mut winnerFlag: i32 = 0;
    let mut history: Vec<String> = Vec::new();

    while (winnerFlag == 0) {
        println!("Please choose your column player {} : ", player);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let number: i32 = input.trim().parse().unwrap();
        let column = number - 1;
        print!("\x1B[2J\x1B[1;1H");
        if (player == 1) {
            for i in (0..columns).rev() {
                if (i == column as usize) {
                    for j in (0..rows).rev() {
                        if (j == 0
                            && (gameboard[j][column as usize] == '●'
                                || gameboard[j][column as usize] == '◌'))
                        {
                            print_board(rows, columns, *gameboard);
                            println!("Taken! Choose another column!");
                            break;
                        }
                        if (gameboard[j][column as usize] != '●'
                            && gameboard[j][column as usize] != '◌')
                        {
                            gameboard[j][column as usize] = '●';
                            history.push(format!("Player 1 choose {} column", column + 1));
                            print_board(rows, columns, *gameboard);
                            print_history(&mut history);

                            //check if winner in row
                            winnerFlag = row_winner(player, *gameboard, column, rows);
                            if (winnerFlag == 1) {
                                break;
                            }
                            //check if winner in column
                            winnerFlag = column_winner(player, *gameboard, columns, rows);
                            if (winnerFlag == 1) {
                                break;
                            }
                            //check if winner in diagonal to left
                            winnerFlag = diagonalLeft_winner(player, *gameboard, columns, rows);
                            if (winnerFlag == 1) {
                                break;
                            }
                            //check if winner in diagonal to right
                            winnerFlag = diagonalRight_winner(player, *gameboard, columns, rows);
                            if (winnerFlag == 1) {
                                break;
                            }
                            //check draw
                            if (draw(*gameboard, columns, rows)) {
                                println!("Its a draw!");
                                winnerFlag = 1;
                                break;
                            }

                            player = 2;
                            break;
                        }
                    }
                }
            }
        } else {
            for i in (0..columns).rev() {
                if (i == column as usize) {
                    for j in (0..rows).rev() {
                        if (j == 0
                            && (gameboard[j][column as usize] == '●'
                                || gameboard[j][column as usize] == '◌'))
                        {
                            print_board(rows, columns, *gameboard);
                            println!("Taken! Choose another column!");
                            break;
                        }
                        if (gameboard[j][column as usize] != '●'
                            && gameboard[j][column as usize] != '◌')
                        {
                            gameboard[j][column as usize] = '◌';
                            history.push(format!("Player 2 choose {} column", column + 1));
                            print_board(rows, columns, *gameboard);
                            print_history(&mut history);

                            //check if winner in row
                            winnerFlag = row_winner(player, *gameboard, column, rows);
                            if (winnerFlag == 1) {
                                break;
                            }
                            //check if winner in column
                            winnerFlag = column_winner(player, *gameboard, columns, rows);
                            if (winnerFlag == 1) {
                                break;
                            }
                            //check if winner in diagonal to left
                            winnerFlag = diagonalLeft_winner(player, *gameboard, columns, rows);
                            if (winnerFlag == 1) {
                                break;
                            }
                            //check if winner in diagonal to right
                            winnerFlag = diagonalRight_winner(player, *gameboard, columns, rows);
                            if (winnerFlag == 1) {
                                break;
                            }
                            //check draw
                            if (draw(*gameboard, columns, rows)) {
                                println!("Its a draw!");
                                winnerFlag = 1;
                                break;
                            }

                            player = 1;
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn print_board(rows: usize, columns: usize, gameboard: [[char; 20]; 20]) {
    println!("******CONNECT 4******");
    for i in 0..rows {
        for j in 0..columns {
            print!(" {} ", gameboard[i as usize][j as usize]);
        }
        println!();
    }
}

fn print_history(history: &mut Vec<String>) {
    println!();
    println!("History of moves:");
    for element in history.iter() {
        println!("{}", element);
    }
}

fn row_winner(player: i32, gameboard: [[char; 20]; 20], column: i32, rows: usize) -> i32 {
    if (player == 1) {
        for j in 0..rows - 3 {
            if (gameboard[j][column as usize] == '●'
                && gameboard[j + 1][column as usize] == '●'
                && gameboard[j + 2][column as usize] == '●'
                && gameboard[j + 3][column as usize] == '●')
            {
                println!("****** Player 1 is The Winner! ******");
                return 1;
            }
        }
    } else {
        for j in 0..rows - 3 {
            if (gameboard[j][column as usize] == '◌'
                && gameboard[j + 1][column as usize] == '◌'
                && gameboard[j + 2][column as usize] == '◌'
                && gameboard[j + 3][column as usize] == '◌')
            {
                println!("****** Player 2 is The Winner! ******");
                return 1;
            }
        }
    }
    return 0;
}

fn column_winner(player: i32, gameboard: [[char; 20]; 20], columns: usize, rows: usize) -> i32 {
    if (player == 1) {
        for j in 0..rows {
            for i in 0..columns - 3 {
                if (gameboard[j][i] == '●'
                    && gameboard[j][i + 1] == '●'
                    && gameboard[j][i + 2] == '●'
                    && gameboard[j][i + 3] == '●')
                {
                    println!("****** Player 1 is The Winner! ******");
                    return 1;
                }
            }
        }
    } else {
        for j in 0..rows {
            for i in 0..columns - 3 {
                if (gameboard[j][i] == '◌'
                    && gameboard[j][i + 1] == '◌'
                    && gameboard[j][i + 2] == '◌'
                    && gameboard[j][i + 3] == '◌')
                {
                    println!("****** Player 2 is The Winner! ******");
                    return 1;
                }
            }
        }
    }
    return 0;
}

fn diagonalLeft_winner(
    player: i32,
    gameboard: [[char; 20]; 20],
    columns: usize,
    rows: usize,
) -> i32 {
    if (player == 1) {
        for j in 0..rows - 3 {
            for i in 0..columns {
                if (gameboard[j][i] == '●'
                    && gameboard[j + 1][i + 1] == '●'
                    && gameboard[j + 2][i + 2] == '●'
                    && gameboard[j + 3][i + 3] == '●')
                {
                    println!("****** Player 1 is The Winner! ******");
                    return 1;
                }
            }
        }
    } else {
        for j in 0..rows - 3 {
            for i in 0..columns {
                if (gameboard[j][i] == '◌'
                    && gameboard[j + 1][i + 1] == '◌'
                    && gameboard[j + 2][i + 2] == '◌'
                    && gameboard[j + 3][i + 3] == '◌')
                {
                    println!("****** Player 2 is The Winner! ******");
                    return 1;
                }
            }
        }
    }
    return 0;
}

fn diagonalRight_winner(
    player: i32,
    gameboard: [[char; 20]; 20],
    columns: usize,
    rows: usize,
) -> i32 {
    if (player == 1) {
        for j in (3..rows).rev() {
            for i in 0..columns {
                if (gameboard[j][i] == '●'
                    && gameboard[j - 1][i + 1] == '●'
                    && gameboard[j - 2][i + 2] == '●'
                    && gameboard[j - 3][i + 3] == '●')
                {
                    println!("****** Player 1 is The Winner! ******");
                    return 1;
                }
            }
        }
    } else {
        for j in (3..rows).rev() {
            for i in 0..columns {
                if (gameboard[j][i] == '◌'
                    && gameboard[j - 1][i + 1] == '◌'
                    && gameboard[j - 2][i + 2] == '◌'
                    && gameboard[j - 3][i + 3] == '◌')
                {
                    println!("****** Player 2 is The Winner! ******");
                    return 1;
                }
            }
        }
    }
    return 0;
}

fn draw(gameboard: [[char; 20]; 20], columns: usize, rows: usize) -> bool {
    for i in 0..rows {
        for j in 0..columns {
            if (gameboard[i][j] != '◌' && gameboard[i][j] != '●') {
                return false;
            }
        }
    }
    return true;
}

fn save(history: &mut Vec<String>) {
    let data = history
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    /* let mut full_file_name = file_name + ".txt";
    let mut create_file = File::create(full_file_name).unwrap();
    let mut file = OpenOptions::new().write(true).open("save.txt").unwrap();
    file.write_all(data.as_bytes()).unwrap(); */
    /* let mut file = File::create("./save.txt").expect("cant make file");
    file.write_all(data.as_bytes()).unwrap(); */
    //fs::write(file_name + ".txt", data).expect("Unable to write file");
    //process::exit(1);
    let mut file = OpenOptions::new()
        .write(true)
        .open("C:/Users/cutec/Desktop/Rust/connect_4_game/src/save.txt")
        .unwrap();
    file.write_all(b"PLACEHOLDER")
        .expect("Something went wrong opening the file");
}
