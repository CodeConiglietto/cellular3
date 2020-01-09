use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::Color as GGColor;
use ggez::graphics::DrawMode;
use ggez::graphics::DrawParam;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ndarray::{Array2, ArrayView2};
use rand::random;
use rayon::prelude::*;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

const BLACK: Color = Color { r: 0, g: 0, b: 0 };
const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

impl From<Color> for GGColor {
    fn from(c: Color) -> GGColor {
        GGColor {
            r: c.r as f32 / 255.0,
            g: c.g as f32 / 255.0,
            b: c.b as f32 / 255.0,
            a: 1.0,
        }
    }
}

struct Rule {
    survive_neighbours: [i32; 8],
    born_neighbours: [i32; 8],
}

//One of these per colour
struct RuleSet {
    red_rule: Rule,
    green_rule: Rule,
    blue_rule: Rule,
    cyan_rule: Rule,
    magenta_rule: Rule,
    yellow_rule: Rule,
}

struct SimpleColor {
    has_black: bool,
    has_red: bool,
    has_green: bool,
    has_blue: bool,
    has_cyan: bool,
    has_magenta: bool,
    has_yellow: bool,
    has_white: bool
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

    red_rule: RuleSet,
    green_rule: RuleSet,
    blue_rule: RuleSet,
    cyan_rule: RuleSet,
    magenta_rule: RuleSet,
    yellow_rule: RuleSet,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let (pixels_x, pixels_y) = ggez::graphics::size(ctx);

        let cells_x = 160;
        let cells_y = 90;

        let cell_width = pixels_x as f32 / cells_x as f32;
        let cell_height = pixels_y as f32 / cells_y as f32;

        MyGame {
            // ...
            square: Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(0.0, 0.0, cell_width, cell_height),
                WHITE.into(),
            )
            .unwrap(),

            current_tic: 0,

            bounds: Rect::new(0.0, 0.0, pixels_x, pixels_y),
            cell_array: Array2::from_shape_fn((cells_x, cells_y), |(_x, _y)| -> Color {
                get_random_color()
            }),
            old_cell_array: Array2::from_shape_fn((cells_x, cells_y), |(_x, _y)| -> Color {
                get_random_color()
            }),
        }
    }
}

fn has_red(c: Color) -> bool {
    c.r == 255
}
fn has_green(c: Color) -> bool {
    c.g == 255
}
fn has_blue(c: Color) -> bool {
    c.b == 255
}

fn give_red(c: Color) -> Color {
    Color {
        r: 255,
        g: c.g,
        b: c.b,
    }
}

fn give_green(c: Color) -> Color {
    Color {
        r: c.r,
        g: 255,
        b: c.b,
    }
}

fn give_blue(c: Color) -> Color {
    Color {
        r: c.r,
        g: c.g,
        b: 255,
    }
}

fn take_red(c: Color) -> Color {
    Color {
        r: 0,
        g: c.g,
        b: c.b,
    }
}
fn take_green(c: Color) -> Color {
    Color {
        r: c.r,
        g: 0,
        b: c.b,
    }
}
fn take_blue(c: Color) -> Color {
    Color {
        r: c.r,
        g: c.g,
        b: 0,
    }
}

fn get_random_color() -> Color {
    Color {
        r: if random::<bool>() { 255 } else { 0 },
        g: if random::<bool>() { 255 } else { 0 },
        b: if random::<bool>() { 255 } else { 0 },
    }
}

//This function assumes an x and y between the ranges -dim().<dimension>..infinity
fn wrap_point_to_cell_array(old_cell_array: ArrayView2<'_, Color>, x: i32, y: i32) -> (i32, i32) {
    let width = old_cell_array.dim().0 as i32;
    let height = old_cell_array.dim().1 as i32;

    ((x + width) % width, (y + height) % height)
}

//Get the alive neighbours surrounding x,y in a moore neighbourhood
fn get_alive_neighbours(
    old_cell_array: ArrayView2<'_, Color>,
    x: i32,
    y: i32,
) -> (i32, i32, i32, i32, i32, i32) {
    let mut alive_red_neighbours = 0;
    let mut alive_green_neighbours = 0;
    let mut alive_blue_neighbours = 0;
    let mut alive_cyan_neighbours = 0;
    let mut alive_magenta_neighbours = 0;
    let mut alive_yellow_neighbours = 0;

    for xx in -1..2 {
        for yy in -1..2 {
            if !(xx == 0 && yy == 0) {
                let offset_point = wrap_point_to_cell_array(old_cell_array, x + xx, y + yy);

                let neighbour_color =
                    old_cell_array[[offset_point.0 as usize, offset_point.1 as usize]];

                if has_red(neighbour_color) {
                    alive_red_neighbours += 1;

                    if has_blue(neighbour_color) {
                        alive_magenta_neighbours += 1;
                    }
                }
                if has_green(neighbour_color) {
                    alive_green_neighbours += 1;

                    if has_red(neighbour_color) {
                        alive_yellow_neighbours += 1;
                    }
                }
                if has_blue(neighbour_color) {
                    alive_blue_neighbours += 1;

                    if has_green(neighbour_color) {
                        alive_cyan_neighbours += 1;
                    }
                }
            }
        }
    }

    (
        alive_red_neighbours,
        alive_green_neighbours,
        alive_blue_neighbours,
        alive_cyan_neighbours,
        alive_magenta_neighbours,
        alive_yellow_neighbours,
    )
}

//Get the next state for a cell
fn get_next_color(old_cell_array: ArrayView2<'_, Color>, x: i32, y: i32) -> Color {
    let (alive_red_neighbours, alive_green_neighbours, alive_blue_neighbours, alive_cyan_neighbours, alive_magenta_neighbours, alive_yellow_neighbours) =
        get_alive_neighbours(old_cell_array, x, y);

    //let old_color = self.old_cell_array[[x as usize, y as usize]];
    //let current_color = self.cell_array[[x as usize, y as usize]];

    let old_color = old_cell_array[[x as usize, y as usize]];
    let mut new_color = old_color;

    //Run red sim
    if has_red(old_color)
    //Red cell is alive
    {
        if alive_red_neighbours == 2 || alive_red_neighbours == 3 {
            new_color = give_red(new_color); //Cell survives
        } else {
            new_color = take_red(new_color); //Cell dies of overpopulation or starvation
        }
    } else {
        if alive_red_neighbours == 3 || alive_cyan_neighbours == 3 {
            new_color = give_red(new_color); //Cell is born
        } else {
            new_color = take_red(new_color); //Cell remains dead
        }
    }

    //run green sim
    if has_green(old_color)
    //Green cell is alive
    {
        if alive_green_neighbours == 2 || alive_green_neighbours == 3
        {
            new_color = give_green(new_color); //Cell survives
        } else {
            new_color = take_green(new_color); //Cell dies of overpopulation or starvation
        }
    } else {
        if alive_green_neighbours == 3 || alive_magenta_neighbours == 3 {
            new_color = give_green(new_color); //Cell is born
        } else {
            new_color = take_green(new_color); //Cell remains dead
        }
    }

    //run blue sim
    if has_blue(old_color)
    //Blue cell is alive
    {
        if alive_blue_neighbours == 2 || alive_blue_neighbours == 3 
        {
            new_color = give_blue(new_color); //Cell survives
        } else {
            new_color = take_blue(new_color); //Cell dies of overpopulation or starvation
        }
    } else {
        if alive_blue_neighbours == 3 || alive_yellow_neighbours == 3 {
            new_color = give_blue(new_color); //Cell is born
        } else {
            new_color = take_blue(new_color); //Cell remains dead
        }
    }

    new_color
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let width = self.cell_array.dim().0 as i32;
        let height = self.cell_array.dim().1 as i32;

        let old_cell_array_view = self.old_cell_array.view();

        let active_cells: i32 = ndarray::Zip::indexed(&self.old_cell_array)
            .and(&mut self.cell_array)
            .into_par_iter()
            .map(|((x, y), old, new)| {
                let new_color = get_next_color(old_cell_array_view, x as i32, y as i32);

                //Two checks are necessary to avoid two tic oscillators being counted as active cells
                let older = *new;
                *new = new_color;

                if new_color != older && new_color != *old {
                    1
                } else {
                    0
                }
            })
            .sum();

        if active_cells < width + height {
            for _i in 0..random::<i32>() % width + height {
                self.cell_array[[
                    random::<usize>() % width as usize,
                    random::<usize>() % height as usize,
                ]] = get_random_color();
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
            graphics::draw(
                ctx,
                &self.square,
                DrawParam {
                    dest: [x as f32 * cell_width, y as f32 * cell_height].into(),
                    color: color.into(),
                    ..DrawParam::default()
                },
            )?;
        }

        graphics::present(ctx)
    }
}
