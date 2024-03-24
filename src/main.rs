// Title: Tic Tac Toe
// Created by: Braily Guzman
// Created on: 2024-03-21

use std::{
    io::{self, Write},
    thread::sleep,
};

struct Game {
    user: String,
    computer: String,
    table: Vec<Vec<String>>,
}

impl Game {
    fn print_table(&self) {
        for row in &self.table {
            println!("{:?}", row);
        }
    }

    fn user_turn(&mut self, x: usize, y: usize) {
        self.table[x][y] = self.user.clone();
    }

    fn computer_turn(&mut self) {
        let mut x: usize;
        let mut y: usize;
        loop {
            x = rand::random::<usize>() % 3;
            y = rand::random::<usize>() % 3;
            if self.table[x][y] == " " {
                break;
            }
        }
        self.table[x][y] = self.computer.clone();
    }

    fn check_winner(&self, player: &str) -> bool {
        // Check rows
        for row in 0..3 {
            if &self.table[row][0] == player
                && &self.table[row][1] == player
                && &self.table[row][2] == player
            {
                return true;
            }
        }

        // Check columns
        for col in 0..3 {
            if &self.table[0][col] == player
                && &self.table[1][col] == player
                && &self.table[2][col] == player
            {
                return true;
            }
        }

        // Check diagonals
        if &self.table[0][0] == player && &self.table[1][1] == player && &self.table[2][2] == player
        {
            return true;
        }

        if &self.table[0][2] == player && &self.table[1][1] == player && &self.table[2][0] == player
        {
            return true;
        }

        return false;
    }

    fn check_draw(&self) -> bool {
        for row in &self.table {
            for cell in row {
                if cell == " " {
                    return false;
                }
            }
        }
        return true;
    }

    fn check_computer_winner(&self) -> bool {
        let winner: bool = self.check_winner(&self.computer);
        return winner;
    }

    fn check_user_winner(&self) -> bool {
        let winner: bool = self.check_winner(&self.user);
        return winner;
    }
}

fn main() {
    clear_console();
    let mut game = Game {
        user: "X".to_string(),
        computer: "O".to_string(),
        table: vec![vec![" ".to_string(); 3]; 3],
    };
    game.print_table();

    loop {
        let x: usize;
        let y: usize;

        clear_console();
        game.print_table();

        print!("Enter x: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        x = input.trim().parse().unwrap();

        print!("Enter y: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        y = input.trim().parse().unwrap();

        if x < 1 || x > 3 || y < 1 || y > 3 {
            println!("Invalid move");
            sleep(std::time::Duration::from_secs(2));
            continue;
        }

        let x_index = x - 1;
        let y_index = y - 1;

        if game.table[x_index][y_index] != " " {
            println!("Position already taken!");
            sleep(std::time::Duration::from_secs(2));
            continue;
        }

        game.user_turn(x_index, y_index);
        let user_winner = game.check_user_winner();
        let draw = game.check_draw();

        if draw {
            clear_console();
            game.print_table();
            println!("Draw!");
            sleep(std::time::Duration::from_secs(2));
            break;
        }

        if user_winner {
            clear_console();
            game.print_table();
            println!("User wins!");
            sleep(std::time::Duration::from_secs(2));
            break;
        }

        game.computer_turn();
        let computer_winner = game.check_computer_winner();
        let draw = game.check_draw();

        if draw {
            clear_console();
            game.print_table();
            println!("Draw!");
            sleep(std::time::Duration::from_secs(2));
            break;
        }

        if computer_winner {
            clear_console();
            game.print_table();
            println!("Computer wins!");
            sleep(std::time::Duration::from_secs(2));
            break;
        }
    }

    println!("Game Over!");
}

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
