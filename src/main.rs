#[allow(non_snake_case)]
mod board;
mod gui;
use sdl2::event::Event;

use std::time::Duration;
fn main() {
    let mut b = board::Board::new();
    b.boardFromFen();
    b.print();

    let mut g = gui::Gui::new(704);
    let mut firstSquare = [8, 8];
    'running: loop {
        for event in g.eventPump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::MouseButtonDown{x, y, mouse_btn, ..} => {
                    let x = x * 8 / g.windowres as i32;
                    let y = y * 8 / g.windowres as i32;
                    firstSquare = [x, y];
                },
                Event::MouseButtonUp{x, y, mouse_btn, ..} => {
                    let x = x * 8 / g.windowres as i32;
                    let y = y * 8 / g.windowres as i32;
                    // stayed on same square
                    if firstSquare == [x, y] {
                        if mouse_btn == sdl2::mouse::MouseButton::Left {
                            // deselect square
                            if b.selSquare == [x, y] {
                                b.selSquare = [8, 8];
                            } else if b.selSquare == [8, 8] {
                                b.selSquare = [x, y];
                            } else {
                                b.makeMove(b.selSquare, firstSquare);
                                b.selSquare = [8, 8];
                            }

                        } else {
                            // already highligted
                            if b.highlightedSquares.contains(&[x, y]) {
                                // remove
                                let index = b.highlightedSquares.iter().position(|x| *x == firstSquare).unwrap();
                                b.highlightedSquares.remove(index);
                            } else {
                                b.highlightedSquares.push([x, y]);

                            }
                        }
                    }

                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        g.draw(&b);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
