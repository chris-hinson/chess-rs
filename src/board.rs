use cursive::view::View;
use cursive::Printer;
use cursive::Vec2;

static BLACK_K: &str = "♚";
static WHITE_K: &str = "♔";
static BLACK_Q: &str = "♛";
static WHITE_Q: &str = "♕";
static BLACK_R: &str = "♜";
static WHITE_R: &str = "♖";
static BLACK_B: &str = "♝";
static WHITE_B: &str = "♗";
static BLACK_N: &str = "♞";
static WHITE_N: &str = "♘";
static BLACK_P: &str = "♟";
static WHITE_P: &str = "♙";

pub struct ChessBoard<'a> {
    pub state: [[&'a str; 8]; 8],
}

impl ChessBoard<'static> {
    pub fn new() -> Self {
        let mut board = [[" "; 8]; 8];

        board[0][0] = WHITE_R;
        board[0][1] = WHITE_N;
        board[0][2] = WHITE_B;
        board[0][3] = WHITE_Q;
        board[0][4] = WHITE_K;
        board[0][5] = WHITE_B;
        board[0][6] = WHITE_N;
        board[0][7] = WHITE_R;

        board[7][0] = BLACK_R;
        board[7][1] = BLACK_N;
        board[7][2] = BLACK_B;
        board[7][3] = BLACK_Q;
        board[7][4] = BLACK_K;
        board[7][5] = BLACK_B;
        board[7][6] = BLACK_N;
        board[7][7] = BLACK_R;

        //for (_i, mut col) in board[1].iter_mut().enumerate() {
        for i in 0..8 {
            //col = &mut &*BLACK_P;
            board[1][i] = WHITE_P;
            board[6][i] = BLACK_P
        }

        Self { state: board }
    }

    pub fn execute_move(&mut self, cur_move: &str) {
        let prev_state = self.state[7][0];
        self.state[6][0] = " ";
        self.state[5][0] = prev_state;
    }

    pub fn check_move(&mut self, cur_move: &str) -> bool {
        if cur_move.eq("test") {
            return true;
        } else {
            return false;
        }
    }
}

impl View for ChessBoard<'static> {
    fn draw(&self, printer: &Printer) {
        printer.print((0, 0), "  a  b  c  d  e  f  g  h");
        printer.print((0, 1), " ┏━━┳━━┳━━┳━━┳━━┳━━┳━━┳━━┓");
        //print this 8 times

        for i in 0..7 {
            printer.print(
                (0, 2 + (2 * i) + 0),
                &format!(
                    "{}┃{} ┃{} ┃{} ┃{} ┃{} ┃{} ┃{} ┃{} ┃",
                    i + 1,
                    self.state[i][0],
                    self.state[i][1],
                    self.state[i][2],
                    self.state[i][3],
                    self.state[i][4],
                    self.state[i][5],
                    self.state[i][6],
                    self.state[i][7],
                )
                .to_owned(),
            );
            printer.print((0, 2 + (2 * i) + 1), " ┣━━╋━━╋━━╋━━╋━━╋━━╋━━╋━━┫");
        }

        //TODO: this hurts me to look at... surely theres a better way??
        printer.print(
            (0, 16),
            &format!(
                "{}┃{} ┃{} ┃{} ┃{} ┃{} ┃{} ┃{} ┃{} ┃",
                8,
                self.state[7][0],
                self.state[7][1],
                self.state[7][2],
                self.state[7][3],
                self.state[7][4],
                self.state[7][5],
                self.state[7][6],
                self.state[7][7],
            )
            .to_owned(),
        );

        printer.print((0, 17), " ┗━━┻━━┻━━┻━━┻━━┻━━┻━━┻━━┛");
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(26, 18)
    }
}
