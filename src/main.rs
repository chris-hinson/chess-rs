use cursive::view::Nameable;
use cursive::views::Button;
use cursive::views::Dialog;
use cursive::views::EditView;
use cursive::views::LinearLayout;
use cursive::Cursive;

mod board;
use crate::board::ChessBoard;

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', Cursive::quit);

    let mut board = ChessBoard::new();
    let buttons = LinearLayout::horizontal()
        .child(Button::new("fuck!", quit_func))
        .child(Button::new("move", get_move));
    let view = Dialog::around(
        LinearLayout::vertical()
            .child(board.with_name("board"))
            .child(buttons),
    )
    .title("chess");

    siv.add_layer(view);

    siv.run();
}

fn quit_func(s: &mut Cursive) {
    s.quit();
}

fn get_move(s: &mut Cursive) {
    s.add_layer(
        Dialog::new()
            .title("enter move")
            .content(EditView::new().on_submit(execute_move).with_name("move"))
            .button("submit", |s| {}),
    )
}

fn execute_move(s: &mut Cursive, _move: &str) {
    if _move.is_empty()
        || !(s
            .call_on_name("board", |board: &mut ChessBoard| board.check_move(_move))
            .unwrap())
    {
        s.add_layer(Dialog::info("Please enter a valid move!"));
    } else {
        s.call_on_name("board", |board: &mut ChessBoard| board.execute_move(_move));
        s.pop_layer();
    }
}
