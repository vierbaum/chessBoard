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
         self.moves.push([from, to]);
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

    pub fn genFen(&self) -> String {
        let mut fen = String::new();
        let mut spaceCount = 0;

        for y in 0..8 {
            for x in 0..8 {
                if self.board[x][y] == '0' {
                    spaceCount += 1;
                }
                else if self.board[x][y] != '0' {
                    if spaceCount > 0 {
                        fen.push((spaceCount + 48) as u8 as char);
                    }
                    fen.push(self.board[x][y]);
                    spaceCount = 0;
                }

            }
            if spaceCount > 0 {
                fen.push((spaceCount + 48) as u8 as char);
            }
            fen.push('/');
            spaceCount = 0;
        }

        return fen;

    }

    pub fn genSemiLegalMoves(&mut self, piece: [i32; 2]) -> Vec<[i32;2]> {
        let mut moves = Vec::new();

        let pieceid = self.board[piece[0] as usize][piece[1] as usize];

        if pieceid == 'K' || pieceid == 'k' {
            for x in piece[0] - 1 .. piece[0] + 2 {
                for y in piece[1] - 1 .. piece[1] + 2 {
                    if x < 8 && x >= 0 && y < 8 && y >= 0 && [x, y] != piece {
                        moves.push([x, y]);
                    }
                }
            }
        }

        else if pieceid == 'P' {
            moves.push([piece[0], piece[1] - 1]);
            moves.push([piece[0], piece[1] - 2]);
            moves.push([piece[0] - 1, piece[1] - 1]);
            moves.push([piece[0] + 1, piece[1] - 1]);
        }

        else if pieceid == 'p' {
            moves.push([piece[0], piece[1] + 1]);
            moves.push([piece[0], piece[1] + 2]);
            moves.push([piece[0] - 1, piece[1] + 1]);
            moves.push([piece[0] + 1, piece[1] + 1]);
        }

        else if pieceid == 'N' || pieceid == 'n' {
            moves.push([piece[0] - 1, piece[1] + 2]);
            moves.push([piece[0] + 1, piece[1] + 2]);
            moves.push([piece[0] - 1, piece[1] - 2]);
            moves.push([piece[0] + 1, piece[1] - 2]);

            moves.push([piece[0] - 2, piece[1] + 1]);
            moves.push([piece[0] + 2, piece[1] + 1]);
            moves.push([piece[0] - 2, piece[1] - 1]);
            moves.push([piece[0] + 2, piece[1] - 1]);
        }

        else if pieceid == 'R' || pieceid == 'r' || pieceid == 'Q' || pieceid == 'q' {
            for x in 0..8 {
                moves.push([x, piece[1]]);
            }
            for y in 0..8 {
                moves.push([piece[0], y]);
            }
        }

        if pieceid == 'B' || pieceid == 'b' || pieceid == 'Q' || pieceid == 'q' {
            for x in 0..8 {
                moves.push([x, x - piece[0] + piece[1]]);
                moves.push([x, piece[1] + piece[0] - x]);
            }
        }

        return moves;
    }
}
