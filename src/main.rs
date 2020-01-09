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
            width: 1600.0,
            height: 900.0,
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

        let cells_x = 256;
        let cells_y = 144;

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
    pub fn get_alive_neighbours(&self, x: i32, y: i32) -> (i32, i32, i32) {
        let mut alive_red_neighbours = 0;
        let mut alive_green_neighbours = 0;
        let mut alive_blue_neighbours = 0;

        for xx in -1..2 {
            for yy in -1..2 {
                if !(xx == 0 && yy == 0) {
                    let offset_point = self.wrap_point_to_cell_array(x + xx, y + yy);

                    let neighbour_color =
                        self.old_cell_array[[offset_point.0 as usize, offset_point.1 as usize]];

                    if has_red(neighbour_color) {
                        alive_red_neighbours += 1;
                    }
                    if has_green(neighbour_color) {
                        alive_green_neighbours += 1;
                    }
                    if has_blue(neighbour_color) {
                        alive_blue_neighbours += 1;
                    }
                }
            }
        }

        (alive_red_neighbours, alive_green_neighbours, alive_blue_neighbours)
    }

    fn get_next_color(&self, x: i32, y: i32) -> Color
    {
        let (alive_red_neighbours, alive_blue_neighbours, alive_green_neighbours) = self.get_alive_neighbours(x, y);

        let old_color = self.old_cell_array[[x as usize, y as usize]];
        let current_color = self.cell_array[[x as usize, y as usize]];

        let &mut new_color = WHITE;
        let old_color = self.old_cell_array[[x as usize, y as usize]];
        
        //Run red sim
        if has_red(old_color)//Red cell is alive
        {
            if alive_red_neighbours == 2 || alive_red_neighbours == 3
            {
                new_color = give_red(new_color);//Cell survives
            }else{
                new_color = take_red(new_color);//Cell dies of overpopulation or starvation
            }
        }else{
            if alive_red_neighbours == 3
            {
                new_color = give_red(new_color);//Cell is born
            }else{
                new_color = take_red(new_color);//Cell remains dead
            }
        }

        //run green sim
        if has_green(old_color)//Green cell is alive
        {
            if alive_green_neighbours == 2 || alive_green_neighbours == 3
            {
                new_color = give_green(new_color);//Cell survives
            }else{
                new_color = take_green(new_color);//Cell dies of overpopulation or starvation
            }
        }else{
            if alive_green_neighbours == 3
            {
                new_color = give_green(new_color);//Cell is born
            }else{
                new_color = take_green(new_color);//Cell remains dead
            }
        }

        //run blue sim
        if has_blue(old_color)//Blue cell is alive
        {
            if alive_blue_neighbours == 2 || alive_blue_neighbours == 3
            {
                new_color = give_blue(new_color);//Cell survives
            }else{
                new_color = take_blue(new_color);//Cell dies of overpopulation or starvation
            }
        }else{
            if alive_blue_neighbours == 3
            {
                new_color = give_blue(new_color);//Cell is born
            }else{
                new_color = take_blue(new_color);//Cell remains dead
            }
        }

        *new_color
    }
}

fn has_red(c: Color) -> bool {
    c.r == 1.0
}
fn has_green(c: Color) -> bool {
    c.g == 1.0
}
fn has_blue(c: Color) -> bool {
    c.b == 1.0
}

fn give_red(c: &mut Color) -> &mut Color {
    c.r = 1.0;
    c
}
fn give_green(c: &mut Color) -> &mut Color {
    c.g = 1.0;
    c
}
fn give_blue(c: &mut Color) -> &mut Color {
    c.b = 1.0;
    c
}

fn take_red(c: &mut Color) -> &mut Color {
    c.r = 0.0;
    c
}
fn take_green(c: &mut Color) -> &mut Color {
    c.g = 0.0;
    c
}
fn take_blue(c: &mut Color) -> &mut Color {
    c.b = 0.0;
    c
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let width = self.cell_array.dim().0 as i32;
        let height = self.cell_array.dim().1 as i32;

        let mut active_cells = 0;

        for x in 0..width {
            for y in 0..height {
                let new_color = self.get_next_color(x, y);

                //Two checks are necessary to avoid two tic oscillators being counted as active cells
                if new_color != self.cell_array[[x as usize, y as usize]]
                    && new_color != self.old_cell_array[[x as usize, y as usize]]
                {
                    active_cells += 1;
                }

                self.cell_array[[x as usize, y as usize]] = new_color;
            }
        }

        if active_cells < width + height {
            for _i in 0..random::<i32>() % width + height {
                self.cell_array[[
                    random::<usize>() % width as usize,
                    random::<usize>() % height as usize,
                ]] = BLACK;
            }
        }

        std::mem::swap(&mut self.cell_array, &mut self.old_cell_array);
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
