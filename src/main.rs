use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::DrawParam;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use ggez::graphics::BLACK;
use ggez::graphics::WHITE;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ndarray::Array2;
use rand::random;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("cellular3", "CodeBunny")
        .window_mode(WindowMode {
            width: 1280.0,
            height: 720.0,
            ..WindowMode::default()
        })
        .build()
        .expect("Could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e),
    }
}

struct MyGame {
    //Cell mesh to reuse
    square: Mesh,
    //Tic timer
    current_tic: i32,
    //Screen bounds
    bounds: Rect,
    //The actual cell array
    cell_array: Array2<Color>,
    old_cell_array: Array2<Color>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let (pixels_x, pixels_y) = ggez::graphics::size(ctx);

        let cells_x = 128;
        let cells_y = 72;

        let cell_width = pixels_x as f32 / cells_x as f32;
        let cell_height = pixels_y as f32 / cells_y as f32;

        MyGame {
            // ...
            square: Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, 0.0, cell_width, cell_height),
                BLACK,
            )
            .unwrap(),

            current_tic: 0,

            bounds: Rect::new(0.0, 0.0, pixels_x, pixels_y),
            cell_array: Array2::from_elem((cells_x, cells_y), WHITE),
            old_cell_array: Array2::from_shape_fn((cells_x, cells_y), |(_x, _y)| -> Color {
                if random::<i32>() % 2 == 0 {
                    BLACK //The cell is born alive
                } else {
                    WHITE //The cell is born dead
                }
            }),
        }
    }

    //This function assumes an x and y between the ranges -dim().<dimension>..infinity
    pub fn wrap_point_to_cell_array(&self, x: i32, y: i32) -> (i32, i32) {
        let width = self.cell_array.dim().0 as i32;
        let height = self.cell_array.dim().1 as i32;

        ((x + width) % width, (y + height) % height)
    }

    //TODO: Finish implementing
    pub fn get_alive_neighbours(&self, x: i32, y: i32) -> i32 {
        let mut alive_neighbours = 0;

        for xx in -1..2 {
            for yy in -1..2 {
                let offset_point = self.wrap_point_to_cell_array(x + xx, y + yy);

                if !(xx == 0 && yy == 0)
                    && self.old_cell_array[[offset_point.0 as usize, offset_point.1 as usize]]
                        == BLACK
                {
                    alive_neighbours += 1;
                }
            }
        }

        alive_neighbours
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let width = self.cell_array.dim().0 as i32;
        let height = self.cell_array.dim().1 as i32;

        let mut active_cells = 0;

        for x in 0..width {
            for y in 0..height {
                let alive_neighbours = self.get_alive_neighbours(x as i32, y as i32);

                self.cell_array[[x as usize, y as usize]] =
                    if self.old_cell_array[[x as usize, y as usize]] == BLACK {
                        //The cell is alive
                        if alive_neighbours == 2 || alive_neighbours == 3 {
                            BLACK //A cell survives
                        } else {
                            WHITE //A cell dies of starvation or overpopulation
                        }
                    } else {
                        //The cell is dead
                        if alive_neighbours == 3 {
                            BLACK //The cell is born through reproduction
                        } else {
                            WHITE //The cell remains dead
                        }
                    };

                if self.cell_array[[x as usize, y as usize]]
                    != self.old_cell_array[[x as usize, y as usize]]
                {
                    active_cells += 1;
                }
            }
        }

        std::mem::swap(&mut self.cell_array, &mut self.old_cell_array);

        if active_cells < (width + height) / 2 {
            for _i in 0..random::<i32>() % width + height {
                self.cell_array[[
                    random::<usize>() % width as usize,
                    random::<usize>() % height as usize,
                ]] = BLACK;
            }
        }

        self.current_tic += 1;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let cell_array_width = self.cell_array.dim().0;
        let cell_array_height = self.cell_array.dim().1;

        let cell_width = self.bounds.w as f32 / cell_array_width as f32;
        let cell_height = self.bounds.h as f32 / cell_array_height as f32;

        for ((x, y), &color) in self.cell_array.indexed_iter() {
            if color == BLACK {
                graphics::draw(
                    ctx,
                    &self.square,
                    DrawParam {
                        dest: [x as f32 * cell_width, y as f32 * cell_height].into(),
                        color,
                        ..DrawParam::default()
                    },
                )?;
            }
        }

        graphics::present(ctx)
    }
}
