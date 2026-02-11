use rand::Rng;

struct Play {
    pub x: usize,
    pub y: usize,
    pub score: i32,
}

pub const PERSONNE: i32 = 0;
pub const X: i32 = 1;
pub const O: i32 = 2;

pub struct TicTacToe {
    n: usize,
    grid: Vec<i32>,
    turn: i32,
    turn_number: usize,
    winner: i32,
}

fn shuffle_array(array: &mut Vec<usize>) {
    let mut rnd = rand::rng();
    let mut index: usize;
    let mut temp: usize;
    for i in (0..array.len()).rev() {
        index = rnd.random_range(0..i + 1);
        temp = array[index];
        array[index] = array[i];
        array[i] = temp;
    }
}

impl TicTacToe {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            grid: vec![PERSONNE; n * n],
            turn: X,
            turn_number: 0,
            winner: PERSONNE,
        }
    }

    fn init(&mut self) {
        self.grid = vec![PERSONNE; self.n * self.n];
        self.turn = X;
        self.turn_number = 0;
        self.winner = PERSONNE;
    }
    fn init_player(&mut self, p: i32) {
        self.init();
        self.turn = p;
    }

    pub fn get_n(&self) -> usize {
        return self.n;
    }
    pub fn get_case(&self, x: usize, y: usize) -> i32 {
        return self.grid[y * self.n + x];
    }
    pub fn get_turn(&self) -> i32 {
        return self.turn;
    }
    pub fn get_turn_number(&self) -> usize {
        return self.turn_number;
    }
    pub fn get_winner(&self) -> i32 {
        return self.winner;
    }

    pub fn is_over(&self) -> bool {
        self.turn_number == 9 || self.winner != PERSONNE
    }

    pub fn choose_winner(&self) -> i32 {
        let mut diag1_completed: i32 = self.grid[0];
        let mut diag2_completed: i32 = self.grid[self.n - 1];
        for i in 0..self.n {
            let mut line_completed: i32 = self.grid[i * self.n + 0];
            let mut col_completed: i32 = self.grid[i];
            for j in 0..self.n {
                if line_completed.ne(&self.grid[i * self.n + j]) {
                    line_completed = PERSONNE;
                }
                if col_completed.ne(&self.grid[j * self.n + i]) {
                    col_completed = PERSONNE;
                }
            }
            if line_completed != PERSONNE {
                return line_completed;
            }
            if col_completed != PERSONNE {
                return col_completed;
            }

            if i > 0 {
                if diag1_completed.ne(&self.grid[i * self.n + i]) {
                    diag1_completed = PERSONNE;
                }
                if diag2_completed.ne(&self.grid[i * self.n + self.n - i - 1]) {
                    diag2_completed = PERSONNE;
                }
            }
        }

        if diag1_completed != PERSONNE {
            return diag1_completed;
        }
        if diag2_completed != PERSONNE {
            return diag2_completed;
        }

        PERSONNE
    }

    pub fn play(&mut self, x: usize, y: usize) {
        if self.grid[y * self.n + x] != PERSONNE || self.is_over() {
            return;
        }
        self.grid[y * self.n + x] = self.turn;
        self.turn = 3 - self.turn;
        self.turn_number += 1;
        self.winner = self.choose_winner();
    }

    pub fn remove(&mut self, x: usize, y: usize) {
        if self.grid[y * self.n + x] == PERSONNE {
            return;
        }
        self.grid[y * self.n + x] = PERSONNE;
        self.turn = 3 - self.turn;
        self.turn_number -= 1;
        self.winner = PERSONNE;
    }

    pub fn reset(&mut self) {
        if self.turn_number % 2 == 0 {
            self.init_player(3 - self.turn);
        } else {
            self.init_player(self.turn);
        }
    }

    fn best_play(&mut self, p: i32) -> Option<Play> {
        if self.is_over() {
            return None;
        }
        let mut best_play: Play = Play {
            x: 0,
            y: 0,
            score: i32::MIN,
        };

        let mut rows: Vec<usize> = (0..self.n).collect();
        let mut columns: Vec<usize> = (0..self.n).collect();
        shuffle_array(&mut rows);
        shuffle_array(&mut columns);

        for y in rows {
            for x in columns.clone() {
                if self.grid[y * self.n + x] != PERSONNE {
                    continue;
                }

                let mut score: i32 = i32::MIN;
                self.play(x, y);

                if self.is_over() {
                    if self.winner == PERSONNE {
                        score = 0;
                    } else if self.winner == p {
                        score = i32::MAX
                    } else {
                        score = i32::MIN
                    }
                } else if let Some(temp) = self.best_play(3 - p) {
                    if score < temp.score {
                        score = temp.score;
                    }
                }

                if score > best_play.score {
                    best_play.x = x;
                    best_play.y = y;
                    best_play.score = score;
                }

                self.remove(x, y);
            }
        }

        return Some(Play {
            x: best_play.x,
            y: best_play.y,
            score: -(best_play.score - 1),
        });
    }

    pub fn bot_play(&mut self) {
        if let Some(best_play) = self.best_play(self.turn) {
            self.play(best_play.x, best_play.y);
            println!("bot played {},{}", best_play.x, best_play.y);
        }
    }
}
