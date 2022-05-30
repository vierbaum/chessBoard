#[allow(non_snake_case)]
extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use super::board;

pub struct Gui {
    pub canvas: sdl2::render::WindowCanvas,
    pub windowres: u32,
    squaresize: u32,
    CsquareB: Color,
    CsquareW: Color,
    CsquareHB: Color,
    CsquareHW: Color,
    CsquareSW: Color,
    CsquareSB: Color,
    textureCreator: sdl2::render::TextureCreator<WindowContext>,
    pub eventPump: sdl2::EventPump
}

impl Gui {
    pub fn new(winres: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("chess", winres, winres)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let textureCreator = canvas.texture_creator();
        Gui {
            canvas: canvas,
            windowres: winres,
            squaresize: winres / 8,
            CsquareB: Color::RGB(27, 29, 30),
            CsquareW: Color::RGB(248, 249, 242),
            CsquareHB: Color::RGB(204, 36, 29),
            CsquareHW: Color::RGB(251,73,52),
            CsquareSW: Color::RGB(184, 187, 38),
            CsquareSB: Color::RGB(152, 151, 26),
            textureCreator: textureCreator,
            eventPump: sdl_context.event_pump().unwrap()
        }
    }

    pub fn draw(&mut self, board: &board::Board) {
        for x in 0..8 {
            for y in 0..8 {
                if (x + y) % 2 == 0 {
                    // light
                    self.canvas.set_draw_color(self.CsquareW);
                    if (*board).highlightedSquares.contains(&[x, y]) {
                        self.canvas.set_draw_color(self.CsquareHW);
                    }
                    if [x, y] == (*board).selSquare {
                        self.canvas.set_draw_color(self.CsquareSW);
                    }


                } else {
                    // dark
                    self.canvas.set_draw_color(self.CsquareB);
                    if (*board).highlightedSquares.contains(&[x, y]) {
                        self.canvas.set_draw_color(self.CsquareHB);
                    }
                    if [x, y] == (*board).selSquare {
                        self.canvas.set_draw_color(self.CsquareSB);
                    }

                }

                self.canvas.fill_rect(Rect::new(x * self.squaresize as i32, y * self.squaresize as i32, self.squaresize, self.squaresize)).unwrap();

                if (*board).board[x as usize][y as usize] != '0' {
                    let path = format!("pieces/{}.png", (*board).board[x as usize][y as usize]);
                    let tex = self.textureCreator.load_texture(path).unwrap();
                    self.canvas.copy(&tex, None, Rect::new(x * self.squaresize as i32, y * self.squaresize as i32, self.squaresize, self.squaresize)).unwrap();
                }
            }
        }

        self.canvas.present();
    }
}
