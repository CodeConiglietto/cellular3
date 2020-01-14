use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{self, spritebatch::SpriteBatch, Color as GGColor, DrawParam, Image, Rect},
    timer, Context, ContextBuilder, GameResult,
};
use ndarray::{s, Array2, ArrayView2};
use rand::prelude::*;
use rayon::prelude::*;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div},
};

const MAX_NEIGHBOUR_ARRAY_COUNT: usize = 9;//Use this for array indexes as it counts zero
const MAX_NEIGHBOUR_COUNT: i32 = 8;//Use this for total neighbours excluding zero
const MAX_COLORS: usize = 8;
const TICS_PER_UPDATE: i32 = 12;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PalletteColor {
    Black,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    White,
}

impl PalletteColor {
    fn get_color(&self) -> Color {
        match self {
            PalletteColor::Black => Color { r: 255, g: 0, b: 0 },
            PalletteColor::Red => Color { r: 255, g: 0, b: 0 },
            PalletteColor::Green => Color { r: 0, g: 255, b: 0 },
            PalletteColor::Blue => Color { r: 0, g: 0, b: 255 },
            PalletteColor::Cyan => Color {
                r: 0,
                g: 255,
                b: 255,
            },
            PalletteColor::Magenta => Color {
                r: 255,
                g: 0,
                b: 255,
            },
            PalletteColor::Yellow => Color {
                r: 255,
                g: 255,
                b: 0,
            },
            PalletteColor::White => Color {
                r: 255,
                g: 255,
                b: 255,
            },
        }
    }

    fn to_index(&self) -> usize {
        match self {
            PalletteColor::Black => 0,
            PalletteColor::Red => 1,
            PalletteColor::Green => 2,
            PalletteColor::Blue => 3,
            PalletteColor::Cyan => 4,
            PalletteColor::Magenta => 5,
            PalletteColor::Yellow => 6,
            PalletteColor::White => 7,
        }
    }

    fn from_index(index: usize) -> PalletteColor {
        match index {
            0 => PalletteColor::Black,
            1 => PalletteColor::Red,
            2 => PalletteColor::Green,
            3 => PalletteColor::Blue,
            4 => PalletteColor::Cyan,
            5 => PalletteColor::Magenta,
            6 => PalletteColor::Yellow,
            7 => PalletteColor::White,
            _ => panic!(),
        }
    }

    fn to_composites(&self) -> [bool; 3] {
        match self {
            PalletteColor::Black => [false, false, false],
            PalletteColor::Red => [true, false, false],
            PalletteColor::Green => [false, true, false],
            PalletteColor::Blue => [false, false, true],
            PalletteColor::Cyan => [false, true, true],
            PalletteColor::Magenta => [true, false, true],
            PalletteColor::Yellow => [true, true, false],
            PalletteColor::White => [true, true, true],
        }
    }

    fn from_composites(composites: [bool; 3]) -> PalletteColor {
        match composites {
            [false, false, false] => PalletteColor::Black,
            [true, false, false] => PalletteColor::Red,
            [false, true, false] => PalletteColor::Green,
            [false, false, true] => PalletteColor::Blue,
            [false, true, true] => PalletteColor::Cyan,
            [true, false, true] => PalletteColor::Magenta,
            [true, true, false] => PalletteColor::Yellow,
            [true, true, true] => PalletteColor::White,
        }
    }

    fn has_color(&self, other: PalletteColor) -> bool {
        let mut has_color = false;
        let current_color = self.to_composites();
        let other_color = other.to_composites();

        for i in 0..3 {
            has_color = has_color || (current_color[i] && other_color[i]);
        }

        has_color
    }

    fn give_color(&mut self, other: PalletteColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_composites();
        let other_color = other.to_composites();

        for i in 0..3 {
            new_color[i] = current_color[i] || other_color[i];
        }

        new_color
    }

    fn take_color(&mut self, other: PalletteColor) -> [bool; 3] {
        let mut new_color = [false; 3];
        let current_color = self.to_composites();
        let other_color = other.to_composites();

        for i in 0..3 {
            new_color[i] = !(current_color[i] && other_color[i]);
        }

        new_color
    }
}

#[derive(Clone, Copy)]
struct Rule {
    life_neighbours: [bool; MAX_NEIGHBOUR_ARRAY_COUNT], //How many neighbours we need to be born
    death_neighbours: [bool; MAX_NEIGHBOUR_ARRAY_COUNT], //How many neighbours we need to be killed
}

//One of these per colour
#[derive(Clone, Copy)]
struct RuleSet {
    rules: [Rule; MAX_COLORS],
}

fn generate_random_neighbour_list() -> [bool; MAX_NEIGHBOUR_ARRAY_COUNT] {
    [
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
        random::<bool>(),
    ]
}

fn generate_random_rule() -> Rule {
    Rule {
        life_neighbours: generate_random_neighbour_list(),
        death_neighbours: generate_random_neighbour_list(),
    }
}

fn generate_random_rule_set() -> RuleSet {
    RuleSet {
        rules: [
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
            generate_random_rule(),
        ],
    }
}

fn mutate_rule_set(rule_set: &mut RuleSet) {
    rule_set.rules[random::<usize>() % MAX_COLORS].life_neighbours
        [random::<usize>() % MAX_NEIGHBOUR_ARRAY_COUNT] = random::<bool>();
    rule_set.rules[random::<usize>() % MAX_COLORS].death_neighbours
        [random::<usize>() % MAX_NEIGHBOUR_ARRAY_COUNT] = random::<bool>();
}

struct MyGame {
    //Game draw texture
    image: Image,
    //Screen bounds
    bounds: Rect,
    //The actual cell array
    old_cell_array: Array2<PalletteColor>,
    cell_array: Array2<PalletteColor>,
    new_cell_array: Array2<PalletteColor>,

    rule_sets: [RuleSet; MAX_COLORS],

    //The rolling total used to calculate the average per update instead of per slice
    rolling_update_stat_total: UpdateStat,
    //The average update stat over time, calculated by averaging rolling total and itself once an update
    average_update_stat: UpdateStat,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let (pixels_x, pixels_y) = ggez::graphics::size(ctx);

        let cells_x = 426;
        let cells_y = 240;

        MyGame {
            // ...
            image: Image::solid(ctx, 1, WHITE.into()).unwrap(),

            bounds: Rect::new(0.0, 0.0, pixels_x, pixels_y),

            old_cell_array: Array2::from_shape_fn(
                (cells_x, cells_y),
                |(_x, _y)| -> PalletteColor { get_random_color() },
            ),
            cell_array: Array2::from_shape_fn((cells_x, cells_y), |(_x, _y)| -> PalletteColor {
                get_random_color()
            }),
            new_cell_array: Array2::from_shape_fn(
                (cells_x, cells_y),
                |(_x, _y)| -> PalletteColor { get_random_color() },
            ),

            rule_sets: [
                generate_random_rule_set(),
                generate_random_rule_set(),
                generate_random_rule_set(),
                generate_random_rule_set(),
                generate_random_rule_set(),
                generate_random_rule_set(),
                generate_random_rule_set(),
                generate_random_rule_set(),
            ],

            rolling_update_stat_total: UpdateStat {
                active_cells: 0,
                similar_neighbours: 0,
            },
            average_update_stat: UpdateStat {
                active_cells: 0,
                similar_neighbours: 0,
            },
        }
    }
}

fn get_random_color() -> PalletteColor {
    PalletteColor::from_index(random::<usize>() % MAX_COLORS)
}

//This function assumes an x and y between the ranges -dim().<dimension>..infinity
fn wrap_point_to_cell_array(
    old_cell_array: ArrayView2<'_, PalletteColor>,
    x: i32,
    y: i32,
) -> (i32, i32) {
    let width = old_cell_array.dim().0 as i32;
    let height = old_cell_array.dim().1 as i32;

    ((x + width) % width, (y + height) % height)
}

//Get the alive neighbours surrounding x,y in a moore neighbourhood, this number should not exceed 8
fn get_alive_neighbours(
    old_cell_array: ArrayView2<'_, PalletteColor>,
    x: i32,
    y: i32,
) -> ([usize; MAX_COLORS], i32) {
    let mut alive_neighbours = [0 as usize; MAX_COLORS]; //An array containing neighbour information for each color
    let similar_neighbours = 0;

    for xx in -1..=1 {
        for yy in -1..=1 {
            if !(xx == 0 && yy == 0) {
                let offset_point = wrap_point_to_cell_array(old_cell_array, x + xx, y + yy);

                let neighbour_color =
                    old_cell_array[[offset_point.0 as usize, offset_point.1 as usize]];

                alive_neighbours[neighbour_color.to_index()] += 1;
            }
        }
    }

    (alive_neighbours, similar_neighbours)
}

//Get the next state for a cell
fn get_next_color(
    rule_sets: [RuleSet; MAX_COLORS],
    old_color: PalletteColor,
    alive_neighbours: [usize; MAX_COLORS],
) -> PalletteColor {
    let mut new_color = old_color;

    for i in 0..MAX_COLORS {
        let index_color = PalletteColor::from_index(i);
        let current_rule = rule_sets[new_color.to_index()].rules[i];

        if new_color.has_color(index_color)
        //This color is alive
        {
            //This color is killed
            if current_rule.death_neighbours[alive_neighbours[i]] {
                new_color = PalletteColor::from_composites(new_color.take_color(index_color));
            }
        } else {
            //This color is dead but is being born again
            if current_rule.life_neighbours[alive_neighbours[i]] {
                new_color = PalletteColor::from_composites(new_color.give_color(index_color));
            }
        }
    }

    new_color
}

//Simple color lerp - May be able to find a better one here: https://www.alanzucconi.com/2016/01/06/colour-interpolation/
fn lerp_ggez_color(a: GGColor, b: GGColor, value: f32) -> GGColor {
    GGColor {
        r: a.r + (b.r - a.r) * value,
        g: a.g + (b.g - a.g) * value,
        b: a.b + (b.b - a.b) * value,
        a: 1.0, //We don't care about transparency lerping
    }
}

#[derive(Default, Clone, Copy)]
struct UpdateStat {
    //Update stats are used to determine an approximation of the entropy of the current state
    //Update stats contain two values:
    //-Active cell count
    //--If the active cell count is high, we have a lot of change
    //--If the active cell count is low, we have a small amount of change
    //-Neighbour similarity
    //--If all neighbours are similar, we have close to a flat color
    //--If all neighbours are distinct, we have visual noise
    active_cells: i32,
    similar_neighbours: i32,
}

impl Add<UpdateStat> for UpdateStat {
    type Output = UpdateStat;

    fn add(self, other: UpdateStat) -> UpdateStat {
        UpdateStat {
            active_cells: self.active_cells + other.active_cells,
            similar_neighbours: self.similar_neighbours + other.similar_neighbours,
        }
    }
}

impl Div<i32> for UpdateStat {
    type Output = UpdateStat;

    fn div(self, other: i32) -> UpdateStat {
        UpdateStat {
            active_cells: self.active_cells / other,
            similar_neighbours: self.similar_neighbours / other,
        }
    }
}

impl Sum for UpdateStat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(UpdateStat::default(), |a, b| a + b)
    }
}

impl AddAssign<UpdateStat> for UpdateStat {
    fn add_assign(&mut self, other: UpdateStat) {
        *self = *self + other;
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let width = self.cell_array.dim().0 as i32;
        let height = self.cell_array.dim().1 as i32;

        let slice_height = height / TICS_PER_UPDATE;
        let slice_y = (timer::ticks(ctx) as i32 % TICS_PER_UPDATE) * slice_height;

        let slice_information = s![0..width, slice_y..slice_y + slice_height];

        let current_update_slice = self.cell_array.slice(slice_information);
        let new_update_slice = self.new_cell_array.slice_mut(slice_information);

        let rule_sets = self.rule_sets;
        let cell_array_view = self.cell_array.view();

        let slice_update_stat: UpdateStat = ndarray::Zip::indexed(current_update_slice)
            .and(new_update_slice)
            .into_par_iter()
            .map(|((x, y), current, new)| {
                let neighbour_result = get_alive_neighbours(cell_array_view, x as i32, y as i32 + slice_y);

                let new_color = get_next_color(rule_sets, *current, neighbour_result.0);

                let older_color = *new;
                *new = new_color;

                UpdateStat {
                    //Two checks are necessary to avoid two tic oscillators being counted as active cells
                    active_cells: if new_color != older_color && new_color != *current {
                        1
                    } else {
                        0
                    },
                    similar_neighbours: neighbour_result.1,
                }
            })
            .sum();

        self.rolling_update_stat_total += slice_update_stat;

        let total_cells = width * height;

        if timer::ticks(ctx) as i32 % TICS_PER_UPDATE == 0 {
            self.average_update_stat =
                (self.average_update_stat + self.rolling_update_stat_total) / 2;

            let sqrt_stagnant_cells =
                ((total_cells - slice_update_stat.active_cells) as f32).sqrt() as i32;

            let activity_value = self.average_update_stat.active_cells as f32 / total_cells as f32;
            let similarity_value = self.average_update_stat.similar_neighbours as f32 / (total_cells * MAX_NEIGHBOUR_COUNT) as f32;

            let similarity_value_squared = similarity_value * similarity_value;
            let activity_value_squared = activity_value * activity_value;

            if random::<i32>() % (sqrt_stagnant_cells / 2 + 1) > slice_update_stat.active_cells {
                for _i in 0..random::<i32>() % (sqrt_stagnant_cells + 1) {
                    self.new_cell_array[[
                        random::<usize>() % width as usize,
                        random::<usize>() % height as usize,
                    ]] = get_random_color();
                }
            }

            if similarity_value < random::<f32>() //It's noisy
            || similarity_value_squared > random::<f32>() //It's flat
            || activity_value > random::<f32>() //It's turbulent
            || activity_value_squared < random::<f32>() //It's unchanging
            {
                let mutations = (sqrt_stagnant_cells as f32 * 0.1) as i32 + 1;

                for _i in 0..random::<i32>() % mutations {
                    mutate_rule_set(&mut self.rule_sets[random::<usize>() % MAX_COLORS]);
                }
            }

            //Rotate the three buffers by swapping
            std::mem::swap(&mut self.cell_array, &mut self.old_cell_array);
            std::mem::swap(&mut self.cell_array, &mut self.new_cell_array);
        }

        timer::yield_now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        let mut sprite_batch = SpriteBatch::new(self.image.clone());

        let cell_array_width = self.cell_array.dim().0;
        let cell_array_height = self.cell_array.dim().1;

        let cell_width = self.bounds.w as f32 / cell_array_width as f32;
        let cell_height = self.bounds.h as f32 / cell_array_height as f32;

        let lerp_value =
            (timer::ticks(ctx) as i32 % TICS_PER_UPDATE) as f32 / TICS_PER_UPDATE as f32;

        for x in 0..cell_array_width {
            for y in 0..cell_array_height {
                let old = &self.old_cell_array[[x, y]];
                let current = &self.cell_array[[x, y]];

                let lerped_color = lerp_ggez_color(
                    old.get_color().into(),
                    current.get_color().into(),
                    lerp_value,
                );

                sprite_batch.add(DrawParam {
                    dest: [x as f32 * cell_width, y as f32 * cell_height].into(),
                    scale: [cell_width, cell_height].into(),
                    color: lerped_color,
                    ..DrawParam::default()
                });
            }
        }

        ggez::graphics::draw(ctx, &sprite_batch, DrawParam::default())?;

        graphics::present(ctx)
    }
}
