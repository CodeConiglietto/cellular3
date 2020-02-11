use std::{
    fs,
    iter::Sum,
    ops::{Add, AddAssign, Div},
};

use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode},
    graphics::{self, Color as GgColor, DrawParam, Image as GgImage, Rect, WHITE},
    input::keyboard,
    timer, Context, ContextBuilder, GameResult,
};
use log::{error, info};
use mutagen::{Generatable, Mutatable};
use ndarray::{s, Array3, ArrayView1, ArrayView3, ArrayViewMut1, Axis};
use rand::prelude::*;
use rayon::prelude::*;
use structopt::StructOpt;

use crate::{
    constants::*,
    datatype::{
        colors::{get_average, IntColor},
        continuous::*,
        image::IMAGE_PRELOADER,
    },
    node::{color_nodes::IntColorNodes, Node},
    opts::Opts,
    updatestate::*,
    util::{DeterministicRng, RNG_SEED},
};

mod constants;
mod datatype;
mod node;
mod opts;
mod preloader;
mod updatestate;
mod util;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    setup_logging();

    let opts = Opts::from_args();
    let (mut ctx, mut event_loop) = ContextBuilder::new("cellular3", "CodeBunny")
        .window_mode(WindowMode::default().dimensions(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT))
        .window_setup(WindowSetup::default().vsync(VSYNC))
        .build()
        .expect("Could not create ggez context!");

    let mut my_game = MyGame::new(&mut ctx, opts);

    // Eagerly initialize the image preloader rather than waiting for the first time it's used
    IMAGE_PRELOADER.with(|_| ());

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => info!("Exited cleanly."),
        Err(e) => error!("Error occurred: {}", e),
    }
}

fn setup_logging() {
    let image_error_dispatch = fern::Dispatch::new()
        .level(log::LevelFilter::Off)
        .level_for(datatype::image::MODULE_PATH, log::LevelFilter::Error)
        .chain(fern::log_file("image_errors.log").unwrap());

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.3f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(image_error_dispatch)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}

#[derive(Debug)]
pub struct History {
    cell_arrays: Vec<Array3<u8>>,
}

impl History {
    fn new(array_width: usize, array_height: usize, size: usize) -> Self {
        Self {
            cell_arrays: (0..size)
                .map(|_| init_cell_array(array_width, array_height))
                .collect(),
        }
    }

    fn get_raw(&self, x: usize, y: usize, t: usize) -> ArrayView1<u8> {
        let array = &self.cell_arrays[t % self.cell_arrays.len()];
        array.slice(s![y % array.dim().0, x % array.dim().1, ..])
    }

    fn get(&self, x: usize, y: usize, t: usize) -> IntColor {
        let raw = self.get_raw(x, y, t);
        IntColor {
            r: raw[0],
            g: raw[1],
            b: raw[2],
        }
    }
}

struct MyGame {
    //Screen bounds
    bounds: Rect,

    // The actual cell array
    // Dimensions are [y, x, c]
    history: History,
    next_cell_array: Array3<u8>,

    old_texture: GgImage,
    current_texture: GgImage,

    //rule_sets: [RuleSet; MAX_COLORS],

    //The rolling total used to calculate the average per update instead of per slice
    rolling_update_stat_total: UpdateStat,
    //The average update stat over time, calculated by averaging rolling total and itself once an update
    average_update_stat: UpdateStat,
    //The mechanism responsible for creating an initial state if all automata have died
    //reseeder: Reseeder,
    //The root node for the tree that computes the next screen state
    root_node: Box<IntColorNodes>,

    tree_dirty: bool,
    current_t: usize,
    rng: DeterministicRng,
    opts: Opts,
}

impl MyGame {
    pub fn new(ctx: &mut Context, opts: Opts) -> MyGame {
        // Load/create resources such as images here.
        let (pixels_x, pixels_y) = ggez::graphics::size(ctx);

        let dummy_texture = GgImage::solid(ctx, 1, WHITE).unwrap();

        if let Some(seed) = opts.seed {
            info!("Manually setting RNG seed");
            *RNG_SEED.lock().unwrap() = seed;
        }

        fs::write("last_seed.txt", &RNG_SEED.lock().unwrap().to_string()).unwrap();

        let mut rng = DeterministicRng::new();

        MyGame {
            bounds: Rect::new(0.0, 0.0, pixels_x, pixels_y),

            next_cell_array: init_cell_array(CELL_ARRAY_WIDTH, CELL_ARRAY_HEIGHT),
            history: History::new(
                CELL_ARRAY_WIDTH,
                CELL_ARRAY_HEIGHT,
                CELL_ARRAY_HISTORY_LENGTH,
            ),

            old_texture: dummy_texture.clone(),
            current_texture: dummy_texture,

            // rule_sets: [
            //     generate_random_rule_set(),
            //     generate_random_rule_set(),
            //     generate_random_rule_set(),
            //     generate_random_rule_set(),
            //     generate_random_rule_set(),
            //     generate_random_rule_set(),
            //     generate_random_rule_set(),
            //     generate_random_rule_set(),
            // ],
            rolling_update_stat_total: UpdateStat {
                activity_value: 0.0,
                similarity_value: 0.0,
            },
            average_update_stat: UpdateStat {
                activity_value: 0.0,
                similarity_value: 0.0,
            },

            // reseeder: Reseeder::Modulus {
            //     x_mod: 4,
            //     y_mod: 4,
            //     x_offset: random::<usize>() % CELL_ARRAY_WIDTH,
            //     y_offset: random::<usize>() % CELL_ARRAY_HEIGHT,
            //     color_table: Array2::from_shape_fn((2, 2), |_| get_random_color()),
            // },
            root_node: Box::new(
                IntColorNodes::generate_rng(&mut rng, mutagen::State::default()),
                // PalletteColorNodes::EqColor {
                //     child_a: Box::new(PalletteColorNodes::FromUNFloat {
                //         child: UNFloatNodes::FromSNFloat {
                //             child: Box::new(SNFloatNodes::RidgedMultiNoise { noise: RidgedMulti::new() }),
                //         },
                //     }),
                //     child_b: Box::new(PalletteColorNodes::FromUNFloat {
                //         child: UNFloatNodes::FromSNFloat {
                //             child: Box::new(SNFloatNodes::WorleyNoise {
                //                 noise: Worley::new(),
                //             }),
                //         },
                //     }),
                // },
            ),

            tree_dirty: true,
            current_t: 0,
            rng,
            opts,
        }
    }
}

// fn get_random_color() -> PalletteColor {
//     PalletteColor::from_index(random::<usize>() % MAX_COLORS)
// }

//Get the alive neighbours surrounding x,y in a moore neighbourhood, this number should not exceed 8
// fn get_alive_neighbours(
//     old_cell_array: ArrayView2<'_, PalletteColor>,
//     x: i32,
//     y: i32,
// ) -> ([usize; MAX_COLORS], i32) {
//     let mut alive_neighbours = [0 as usize; MAX_COLORS]; //An array containing neighbour information for each color
//     let mut similar_neighbours = 0;

//     let this_color = old_cell_array[[x as usize, y as usize]];

//     for xx in -1..=1 {
//         for yy in -1..=1 {
//             if !(xx == 0 && yy == 0) {
//                 let offset_point = wrap_point_to_cell_array(old_cell_array, x + xx, y + yy);

//                 let neighbour_color =
//                     old_cell_array[[offset_point.0 as usize, offset_point.1 as usize]];

//                 alive_neighbours[neighbour_color.to_index()] += 1;

//                 if neighbour_color == this_color {
//                     similar_neighbours += 1;
//                 }
//             }
//         }
//     }

//     (alive_neighbours, similar_neighbours)
// }

//Get the next state for a cell
// fn get_next_color(
//     rule_sets: [RuleSet; MAX_COLORS],
//     old_color: PalletteColor,
//     alive_neighbours: [usize; MAX_COLORS],
// ) -> PalletteColor {
//     let mut new_color = old_color;

//     for i in 0..MAX_COLORS {
//         let index_color = PalletteColor::from_index(i);
//         let current_rule = rule_sets[new_color.to_index()].rules[i];

//         if new_color.has_color(index_color)
//         //This color is alive
//         {
//             //This color is killed
//             if current_rule.death_neighbours[alive_neighbours[i]] {
//                 new_color = PalletteColor::from_components(new_color.take_color(index_color));
//             }
//         } else {
//             //This color is dead but is being born again
//             if current_rule.life_neighbours[alive_neighbours[i]] {
//                 new_color = PalletteColor::from_components(new_color.give_color(index_color));
//             }
//         }
//     }

//     new_color
// }

//Simple color lerp - May be able to find a better one here: https://www.alanzucconi.com/2016/01/06/colour-interpolation/
//fn lerp_float_color(a: FloatColor, b: FloatColor, value: f32) -> FloatColor {
//    FloatColor {
//        r: a.r + (b.r - a.r) * value,
//        g: a.g + (b.g - a.g) * value,
//        b: a.b + (b.b - a.b) * value,
//        a: 1.0, //We don't care about transparency lerping
//    }
//}

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
    activity_value: f32,
    similarity_value: f32,
}

impl Add<UpdateStat> for UpdateStat {
    type Output = UpdateStat;

    fn add(self, other: UpdateStat) -> UpdateStat {
        UpdateStat {
            activity_value: self.activity_value + other.activity_value,
            similarity_value: self.similarity_value + other.similarity_value,
        }
    }
}

impl Div<f32> for UpdateStat {
    type Output = UpdateStat;

    fn div(self, other: f32) -> UpdateStat {
        UpdateStat {
            activity_value: self.activity_value / other,
            similarity_value: self.similarity_value / other,
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
        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            self.tree_dirty = true;
        }

        let current_t = self.current_t;

        let slice_height = CELL_ARRAY_HEIGHT / TICS_PER_UPDATE;
        let slice_y = (timer::ticks(ctx) % TICS_PER_UPDATE) * slice_height;
        let slice_y_range = slice_y..slice_y + slice_height;

        let mut new_update_slice = self.next_cell_array.slice_mut(s![slice_y_range, .., ..]);
        let new_update_iter = new_update_slice.lanes_mut(Axis(2));

        let history = &self.history;

        //let rule_sets = self.rule_sets;

        let root_node = &self.root_node;

        let update_step = |y, x, mut new: ArrayViewMut1<u8>| {
            let total_cells = CELL_ARRAY_WIDTH * CELL_ARRAY_HEIGHT;
            // let neighbour_result =
            //     get_alive_neighbours(cell_array_view, x as i32, y as i32 + slice_y);

            let new_color = root_node.compute(UpdateState {
                coordinate_set: CoordinateSet {
                    x: UNFloat::new(x as f32 / CELL_ARRAY_WIDTH as f32).to_signed(),
                    y: UNFloat::new((y + slice_y as usize) as f32 / CELL_ARRAY_HEIGHT as f32)
                        .to_signed(),
                    t: current_t as f32,
                },
                history,
            }); //get_next_color(rule_sets, *current, neighbour_result.0);

            new[0] = new_color.r;
            new[1] = new_color.g;
            new[2] = new_color.b;

            let current_color = history.get(x, y, current_t);
            let older_color = history.get(x, y, usize::max(current_t, 1) - 1);

            UpdateStat {
                //Two checks are necessary to avoid two tic oscillators being counted as active cells
                activity_value: ((get_average(older_color.into()) - get_average(new_color.into()))
                    / total_cells as f32)
                    .abs(),
                similarity_value: 0.0, //neighbour_result.1,
            }
        };

        let zip = ndarray::Zip::indexed(new_update_iter);

        let slice_update_stat: UpdateStat = if PARALLELIZE {
            zip.into_par_iter()
                .map(|((y, x), new)| update_step(y, x, new))
                .sum()
        } else {
            let mut stat = UpdateStat::default();
            zip.apply(|(y, x), new| stat += update_step(y, x, new));
            stat
        };

        self.rolling_update_stat_total += slice_update_stat;

        if timer::ticks(ctx) % TICS_PER_UPDATE == 0 {
            self.average_update_stat =
                (self.average_update_stat + self.rolling_update_stat_total) / 2.0;

            // let sqrt_stagnant_cells =
            //     ((total_cells - slice_update_stat.active_cells) as f32).sqrt() as i32;

            // let similarity_value = self.average_update_stat.similar_neighbours as f32
            //     / (total_cells * MAX_NEIGHBOUR_COUNT) as f32;

            // let similarity_value_squared = similarity_value * similarity_value;
            // let activity_value_squared = activity_value * activity_value;

            // if activity_value < 0.001 || similarity_value > 0.999 {
            //     //if random::<i32>() % (sqrt_stagnant_cells / 2 + 1) > slice_update_stat.active_cells {
            //     //&self.reseeder.reseed(&mut self.new_cell_array);
            //     &self.reseeder.mutate();

            //     mutate_rule_set(&mut self.rule_sets[random::<usize>() % MAX_COLORS]);

            //     // for _i in 0..random::<i32>() % (sqrt_stagnant_cells + 1) {
            //     //     self.new_cell_array[[
            //     //         random::<usize>() % width as usize,
            //     //         random::<usize>() % height as usize,
            //     //     ]] = get_random_color();
            //     // }
            // }

            // if similarity_value < random::<f32>() //It's noisy
            // || similarity_value_squared > random::<f32>() //It's flat
            // || activity_value > random::<f32>() //It's turbulent
            // || activity_value_squared < random::<f32>()
            // //It's unchanging
            // {
            //     let mutations = TICS_PER_UPDATE;

            //     for _i in 0..random::<i32>() % mutations {
            //         mutate_rule_set(&mut self.rule_sets[random::<usize>() % MAX_COLORS]);
            //     }
            // }

            self.rolling_update_stat_total = UpdateStat {
                activity_value: 0.0,
                similarity_value: 0.0,
            };

            if self.tree_dirty
                || self
                    .rng
                    .gen_range(0.0, dbg!(self.average_update_stat.activity_value) as f64)
                    < 0.1
            {
                info!("====TIC: {} MUTATING TREE====", self.current_t);
                self.root_node
                    .mutate_rng(&mut self.rng, mutagen::State::default());
                info!("{:#?}", &self.root_node);
                self.tree_dirty = false;
            }

            std::mem::swap(&mut self.old_texture, &mut self.current_texture);
            self.current_texture = compute_texture(ctx, self.next_cell_array.view());

            // Rotate the buffers by swapping
            let h_len = self.history.cell_arrays.len();
            std::mem::swap(
                &mut self.history.cell_arrays[current_t % h_len],
                &mut self.next_cell_array,
            );

            self.current_t += 1;
        }

        timer::yield_now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let base_params = DrawParam::new().dest([0.0, 0.0]).scale([
            self.bounds.w as f32 / CELL_ARRAY_WIDTH as f32,
            self.bounds.h as f32 / CELL_ARRAY_HEIGHT as f32,
        ]);

        let lerp_value = (timer::ticks(ctx) % TICS_PER_UPDATE) as f32 / TICS_PER_UPDATE as f32;

        ggez::graphics::draw(ctx, &self.old_texture, base_params)?;
        ggez::graphics::draw(
            ctx,
            &self.current_texture,
            base_params.color(GgColor::new(1.0, 1.0, 1.0, lerp_value)),
        )?;

        graphics::present(ctx)?;

        Ok(())
    }
}

fn init_cell_array(width: usize, height: usize) -> Array3<u8> {
    Array3::from_shape_fn(
        (height, width, 4),
        |(_y, _x, c)| if c == 3 { 255 } else { 0 },
    )
}

fn compute_texture(ctx: &mut Context, cell_array: ArrayView3<u8>) -> GgImage {
    let (height, width, _) = cell_array.dim();
    let mut image = GgImage::from_rgba8(
        ctx,
        width as u16,
        height as u16,
        cell_array.as_slice().unwrap(),
    )
    .unwrap();

    image.set_filter(ggez::graphics::FilterMode::Nearest);
    image
}
