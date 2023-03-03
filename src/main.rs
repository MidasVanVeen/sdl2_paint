extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Point, Rect};
use sdl2::{pixels::Color, render::WindowCanvas};

struct BrushStroke {
    beginpos: Point,
    endpos: Point,
    color: Color,
}

struct Brush {
    color: Color,
    colors: [Color; 8],
}

impl Brush {
    fn set_brush(&mut self, brush_id: usize) {
        if brush_id > 7 {
            return;
        }
        self.color = self.colors[brush_id];
    }
}

impl Default for Brush {
    fn default() -> Brush {
        Brush {
            color: Color::BLACK,
            colors: [
                Color::BLACK,
                Color::RED,
                Color::GREEN,
                Color::BLUE,
                Color::MAGENTA,
                Color::YELLOW,
                Color::CYAN,
                Color::WHITE,
            ],
        }
    }
}

struct PaintCanvas {
    rect: Rect,
    background_color: Color,
    strokes: Vec<BrushStroke>,
    brush: Brush,
}

impl PaintCanvas {
    fn clear(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.background_color);
        canvas.fill_rect(self.rect).unwrap();
        for i in 0..8 {
            canvas.set_draw_color(self.brush.colors[i]);
            canvas
                .fill_rect(Rect::new(i as i32 * 50, 0, 50, 50))
                .unwrap()
        }
    }

    fn clear_strokes(&mut self) {
        self.strokes = vec![];
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        for stroke in &self.strokes {
            canvas.set_draw_color(stroke.color);
            canvas.draw_line(stroke.beginpos, stroke.endpos).unwrap();
        }
    }

    fn add(&mut self, p1: Point, p2: Point) {
        self.strokes.push(BrushStroke {
            beginpos: p1,
            endpos: p2,
            color: self.brush.color,
        });
    }
}

impl Default for PaintCanvas {
    fn default() -> PaintCanvas {
        PaintCanvas {
            rect: Rect::new(0, 50, 400, 350),
            background_color: Color::WHITE,
            strokes: vec![],
            brush: Brush::default(),
        }
    }
}

fn main() {
    // initialize sdl2
    let sdl_context = sdl2::init().unwrap();
    #[allow(unused_mut)]
    let mut video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Paint", 400, 400)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // init canvas
    let mut canvas = window.into_canvas().build().unwrap();

    let (mut last_x, mut last_y): (i32, i32) = (0, 0);
    let mut p: PaintCanvas = PaintCanvas::default();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(c), ..
                } => {
                    match c {
                        Keycode::C => p.clear_strokes(),
                        _ => {}
                    };
                }
                _ => {}
            }
        }

        let mouse_state = event_pump.mouse_state();
        if mouse_state.is_mouse_button_pressed(MouseButton::Left) {
            if mouse_state.y() <= 50 {
                p.brush
                    .set_brush((mouse_state.x() as f32 / 50.0).floor() as usize);
            }
            p.add(
                Point::new(last_x, last_y),
                Point::new(mouse_state.x(), mouse_state.y()),
            );
        }

        last_x = mouse_state.x();
        last_y = mouse_state.y();

        p.clear(&mut canvas);
        p.draw(&mut canvas);
        canvas.present();
    }
}
