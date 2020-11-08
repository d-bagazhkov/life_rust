extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ButtonArgs, ButtonEvent, Button, MouseButton, ButtonState, MouseCursorEvent};
use piston::event_loop::{Events, EventSettings};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use graphics::rectangle::Border;
use piston::window::WindowSettings;
use table::*;
use color_rgba::ColorsRGBA::*;

mod table;
mod color_rgba;

const WIDTH: f64 = 1200.0;
const HEIGHT: f64 = 600.0;
const CELL_SIZE: f64 = 30.0;

pub struct App {
    table: Table,
    time: f64,
    view_grid: bool,
    step_time: f64,
}

impl App {

    fn new(table: Table) -> App {
        App {
            table, 
            time: 0.0,
            view_grid: true,
            step_time: 0.2
        }
    }

    fn get_table(&self) -> &Table {
        &self.table
    }

    fn render(&mut self, args: &RenderArgs, glx: &mut GlGraphics) {
        use graphics::*;

        let table = self.get_table();
        let (rows, columns) = self.table.get_size();

        use Cell::*;
        glx.draw(args.viewport(), |c, gl| {
            for row in 0..rows {
                for column in 0..columns {
                    let color = match table.get((row, column)) {
                        Dyaboo => BLUE.get(),
                        _ => WHITE.get()
                    };
                    let square_x = column as f64 * CELL_SIZE; 
                    let square_y = row as f64 * CELL_SIZE; 
                    let mut delta = 0.0;
                    if self.view_grid {
                        delta = 0.5;
                    }
                    Rectangle::new(color)
                        .border(Border {color: BLACK.get(), radius: 0.1} )
                        .draw([square_x + delta, square_y + delta, CELL_SIZE - delta, CELL_SIZE - delta], &c.draw_state, c.transform, gl);
                }
            }
        });
    }

    fn update(&mut self, dt: f64) {
        self.time += dt;
        if self.time < self.step_time {
            return;
        }
        self.time -= self.step_time;
        let (rows, columns) = self.table.get_size();
        let mut updated_table = Table::new(rows, columns);
        for row in 0..rows {
            for column in 0..columns {
                use Cell::*;
                match self.table.get((row, column)) {
                    Void => {
                        let count = self.table.around_count((row, column), Some(Dyaboo));
                        if count == 3 {
                            updated_table.set((row, column), Dyaboo);
                        }
                    },
                    Dyaboo => {
                        let count = self.table.around_count((row, column), Some(Dyaboo));
                        if count > 3 || count < 2 {
                            updated_table.set((row, column), Void);
                        } else {
                            updated_table.set((row, column), Dyaboo);
                        }
                    }
                }
            }
        }
        self.table = updated_table
    }

}

fn fill(table: &mut Table) {
    let (rows, columns) = table.get_size();
    let x_shift = 0;
    let y_shift = 0;
    enum FillGrid {
        Random, Glider, LightweightSpaceship, GosperGun
    }

    let grid = FillGrid::GosperGun;
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    let v = vec![Cell::Void, Cell::Void, Cell::Dyaboo];
    for row in 0..rows {
        for column in 0..columns {
            match grid {
                FillGrid::Random => {
                    let i: usize = rng.gen_range(0, v.len());
                    table.set((row, column), v[i]);
                },
                FillGrid::Glider => {
                    if row == 0 + x_shift && column == 1 + y_shift {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 1 + x_shift && column == 2 + y_shift {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 2 + x_shift && vec![0, 1, 2].into_iter().map(|y| y + y_shift).any(|y| y == column) {
                        table.set((row, column), Cell::Dyaboo);
                    }
                },
                FillGrid::LightweightSpaceship => {
                    if row == 0 + x_shift && vec![1, 2, 3, 4].into_iter().map(|y| y + y_shift).any(|y| y == column) {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 1 + x_shift && vec![0, 4].into_iter().map(|y| y + y_shift).any(|y| y == column) {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 2 + x_shift && column == 4 + y_shift {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 3 + x_shift && column == 3 + y_shift {
                        table.set((row, column), Cell::Dyaboo);
                    }
                },
                FillGrid::GosperGun => {
                    if row == 0 + x_shift && column == 24 + y_shift {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 1 + x_shift && vec![22, 24].into_iter().map(|y| y + y_shift).any(|y| y == column) {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 2 + x_shift && vec![12, 13, 20, 21, 34, 35].into_iter().map(|y| y + y_shift).any(|y| y == column) {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 3 + x_shift && vec![11, 15, 20, 21, 34, 35].into_iter().map(|y| y + y_shift).any(|y| y == column) {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 4 + x_shift && vec![0, 1, 10, 16, 20, 21].into_iter().map(|y| y + y_shift).any(|y| y == column) {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 5 + x_shift && vec![0, 1, 10, 14, 16, 17, 22, 24].into_iter().map(|y| y + y_shift).any(|y| y == column) {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 6 + x_shift && vec![10, 16, 24].into_iter().map(|y| y + y_shift).any(|y| y == column) {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 7 + x_shift && vec![11, 15].into_iter().map(|y| y + y_shift).any(|y| y == column) {
                        table.set((row, column), Cell::Dyaboo);
                    }
                    if row == 8 + x_shift && vec![12, 13].into_iter().map(|y| y + y_shift).any(|y| y == column) {
                        table.set((row, column), Cell::Dyaboo);
                    }
                }
            }
        }
    }
}

#[warn(unused_imports)]
fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Life", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let rows = (HEIGHT / CELL_SIZE) as usize;
    let columns = (WIDTH / CELL_SIZE) as usize;
    let mut table = Table::new(rows, columns);

    fill(&mut table);

    let mut gl = GlGraphics::new(opengl);

    let mut app = App::new(table);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut gl);
        }
        if let Some(update_args) = e.update_args() {
            app.update(update_args.dt);
        }
    }
}