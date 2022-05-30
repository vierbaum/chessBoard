pub struct Board {
    pub board: [[char; 8]; 8],
    pub fen: String,
    pub selSquare: [i32; 2],
    pub moves: Vec<[[i32; 2]; 2]>,
    pub highlightedSquares: Vec<[i32; 2]>
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [['0'; 8]; 8],
            fen: String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            selSquare: [8, 8],
            moves: Vec::new(),
            highlightedSquares: Vec::new()
        }
    }
     pub fn makeMove(&mut self, from: [i32; 2], to: [i32; 2]) {
         self.board[to[0] as usize][to[1] as usize] = self.board[from[0] as usize][from[1] as usize];
         self.board[from[0] as usize][from[1] as usize] = '0';
     }

    pub fn boardFromFen(&mut self) {
        let mut x = 0;
        let mut y = 0;
        for c in self.fen.bytes() {
            if c == 47 { // \/
                y += 1;
                x = 0
            } else if 48 < c && c < 56 {
                // 1..7
                x += c as usize - 48;
            } else if c != 56 {
                // 8
                self.board[x][y] = c as char;
                x += 1

            }
        }
    }

    pub fn print(&self) {
        println!("");
        for y in 0..8 {
            for x in 0..8 {
                print!("{}", self.board[x][y])
            }
            println!("");
        }
    }
}
