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
        colors::{get_average, ByteColor},
        continuous::*,
        image::IMAGE_PRELOADER,
    },
    node::{color_nodes::FloatColorNodes, Node},
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
        .window_mode(
            WindowMode::default()
                .dimensions(CONSTS.initial_window_width, CONSTS.initial_window_height),
        )
        .window_setup(WindowSetup::default().vsync(CONSTS.vsync))
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
        .level(log::LevelFilter::Info)
        .level_for(module_path!(), log::LevelFilter::Trace)
        .chain(image_error_dispatch)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}

#[derive(Debug)]
pub struct HistoryStep {
    cell_array: Array3<u8>,
    computed_texture: GgImage,
}

#[derive(Debug)]
pub struct History {
    history_steps: Vec<HistoryStep>,
}

impl History {
    fn new(ctx: &mut Context, array_width: usize, array_height: usize, size: usize) -> Self {
        Self {
            history_steps: (0..size)
                .map(|_| HistoryStep {
                    cell_array: init_cell_array(array_width, array_height),
                    computed_texture: GgImage::solid(ctx, 1, WHITE).unwrap(),
                })
                .collect(),
        }
    }

    fn get_raw(&self, x: usize, y: usize, t: usize) -> ArrayView1<u8> {
        let array = &self.history_steps[t % self.history_steps.len()].cell_array;
        array.slice(s![y % array.dim().0, x % array.dim().1, ..])
    }

    fn get(&self, x: usize, y: usize, t: usize) -> ByteColor {
        let raw = self.get_raw(x, y, t);
        ByteColor {
            r: raw[0],
            g: raw[1],
            b: raw[2],
        }
    }
}

struct MyGame {
    //Screen bounds
    bounds: Rect,

    history: History,
    next_history_step: HistoryStep,

    //rule_sets: [RuleSet; MAX_COLORS],

    //The rolling total used to calculate the average per update instead of per slice
    rolling_update_stat_total: UpdateStat,
    //The average update stat over time, calculated by averaging rolling total and itself once an update
    average_update_stat: UpdateStat,
    //The root node for the tree that computes the next screen state
    root_node: Box<FloatColorNodes>,

    tree_dirty: bool,
    current_t: usize,
    rng: DeterministicRng,
    opts: Opts,
}

impl MyGame {
    pub fn new(ctx: &mut Context, opts: Opts) -> MyGame {
        // Load/create resources such as images here.
        let (pixels_x, pixels_y) = ggez::graphics::size(ctx);

        if let Some(seed) = opts.seed {
            info!("Manually setting RNG seed");
            *RNG_SEED.lock().unwrap() = seed;
        }

        fs::write("last_seed.txt", &RNG_SEED.lock().unwrap().to_string()).unwrap();

        let mut rng = DeterministicRng::new();

        MyGame {
            bounds: Rect::new(0.0, 0.0, pixels_x, pixels_y),

            next_history_step: HistoryStep {
                cell_array: init_cell_array(CONSTS.cell_array_width, CONSTS.cell_array_height),
                computed_texture: GgImage::solid(ctx, 1, WHITE).unwrap(),
            },
            history: History::new(
                ctx,
                CONSTS.cell_array_width,
                CONSTS.cell_array_height,
                CONSTS.cell_array_history_length,
            ),
            rolling_update_stat_total: UpdateStat {
                activity_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
            },
            average_update_stat: UpdateStat {
                activity_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
            },

            root_node: Box::new(FloatColorNodes::generate_rng(
                &mut rng,
                mutagen::State::default(),
            )),

            tree_dirty: true,
            current_t: 0,
            rng,
            opts,
        }
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
    activity_value: f32,
    local_similarity_value: f32,
    global_similarity_value: f32,
}

impl Add<UpdateStat> for UpdateStat {
    type Output = UpdateStat;

    fn add(self, other: UpdateStat) -> UpdateStat {
        UpdateStat {
            activity_value: self.activity_value + other.activity_value,
            local_similarity_value: self.local_similarity_value + other.local_similarity_value,
            global_similarity_value: self.global_similarity_value + other.global_similarity_value,
        }
    }
}

impl Div<f32> for UpdateStat {
    type Output = UpdateStat;

    fn div(self, other: f32) -> UpdateStat {
        UpdateStat {
            activity_value: self.activity_value / other,
            local_similarity_value: self.local_similarity_value / other,
            global_similarity_value: self.global_similarity_value / other,
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

        let slice_height = CONSTS.cell_array_height / CONSTS.tics_per_update;
        let slice_y = (timer::ticks(ctx) % CONSTS.tics_per_update) * slice_height;
        let slice_y_range = slice_y..slice_y + slice_height;

        let mut new_update_slice =
            self.next_history_step
                .cell_array
                .slice_mut(s![slice_y_range, .., ..]);
        let new_update_iter = new_update_slice.lanes_mut(Axis(2));

        let history = &self.history;

        //let rule_sets = self.rule_sets;

        let root_node = &self.root_node;

        let update_step = |y, x, mut new: ArrayViewMut1<u8>| {
            let total_cells = CONSTS.cell_array_width * CONSTS.cell_array_height;
            // let neighbour_result =
            //     get_alive_neighbours(cell_array_view, x as i32, y as i32 + slice_y);

            let compute_result = root_node.compute(UpdateState {
                coordinate_set: CoordinateSet {
                    x: UNFloat::new(x as f32 / CONSTS.cell_array_width as f32).to_signed(),
                    y: UNFloat::new(
                        (y + slice_y as usize) as f32 / CONSTS.cell_array_height as f32,
                    )
                    .to_signed(),
                    t: current_t as f32,
                },
                history,
            }); //get_next_color(rule_sets, *current, neighbour_result.0);

            let new_color = ByteColor::from(compute_result);

            new[0] = new_color.r;
            new[1] = new_color.g;
            new[2] = new_color.b;

            let current_color = history.get(x, y, current_t);
            let older_color = history.get(x, y, usize::max(current_t, 1) - 1);

            //TODO this should be a random neighbour
            let local_color = history.get(x, y, current_t);
            let global_color = history.get(
                random::<usize>() % CONSTS.cell_array_width,
                random::<usize>() % CONSTS.cell_array_height,
                current_t,
            );

            UpdateStat {
                activity_value: (get_average(older_color.into())
                    - get_average(current_color.into()))
                .abs()
                    / total_cells as f32,
                local_similarity_value: (1.0
                    - (get_average(local_color.into()) - get_average(current_color.into())).abs())
                    / total_cells as f32,
                global_similarity_value: (1.0
                    - (get_average(global_color.into()) - get_average(current_color.into())).abs())
                    / total_cells as f32,
            }
        };

        let zip = ndarray::Zip::indexed(new_update_iter);

        let slice_update_stat: UpdateStat = if CONSTS.parallelize {
            zip.into_par_iter()
                .map(|((y, x), new)| update_step(y, x, new))
                .sum()
        } else {
            let mut stat = UpdateStat::default();
            zip.apply(|(y, x), new| stat += update_step(y, x, new));
            stat
        };

        self.rolling_update_stat_total += slice_update_stat;

        if timer::ticks(ctx) % CONSTS.tics_per_update == 0 {
            self.average_update_stat =
                (self.average_update_stat + self.rolling_update_stat_total) / 2.0;

            dbg!(timer::fps(ctx));

            self.rolling_update_stat_total = UpdateStat {
                activity_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
            };

            if self.tree_dirty
                || (self.rng.gen::<f64>() * dbg!(f64::from(self.average_update_stat.activity_value)) < CONSTS.activity_value_lower_bound
                //|| self.rng.gen::<f64>() * dbg!(f64::from(self.average_update_stat.local_similarity_value)) > CONSTS.local_similarity_upper_bound
                || dbg!(f64::from(self.average_update_stat.global_similarity_value)) >= CONSTS.global_similarity_upper_bound)
                || self.average_update_stat.activity_value > 0.5
            {
                info!("====TIC: {} MUTATING TREE====", self.current_t);
                self.root_node
                    .mutate_rng(&mut self.rng, mutagen::State::default());
                info!("{:#?}", &self.root_node);
                self.tree_dirty = false;
            }

            self.next_history_step.computed_texture =
                compute_texture(ctx, self.next_history_step.cell_array.view());

            // Rotate the buffers by swapping
            let h_len = self.history.history_steps.len();
            std::mem::swap(
                &mut self.history.history_steps[current_t % h_len],
                &mut self.next_history_step,
            );

            self.current_t += 1;
        }

        timer::yield_now();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let base_params = DrawParam::new().dest([0.0, 0.0]).scale([
            self.bounds.w as f32 / CONSTS.cell_array_width as f32,
            self.bounds.h as f32 / CONSTS.cell_array_height as f32,
        ]);

        let lerp_value =
            (timer::ticks(ctx) % CONSTS.tics_per_update) as f32 / CONSTS.tics_per_update as f32;

        let lerp_len = CONSTS.cell_array_lerp_length;

        // let mut alphas = Vec::new();
        for i in 0..lerp_len {
            //let transparency = if i == 0 {1.0} else {if i == 1 {0.5} else {0.0}};
            let alpha = 1.0
                - ((i as f32 - lerp_value) / (lerp_len - 1) as f32)
                    .max(0.0)
                    .powf(4.0);

            let hist_len = self.history.history_steps.len();
            let history_index = (self.current_t + i + hist_len - lerp_len) % hist_len;

            ggez::graphics::draw(
                ctx,
                &self.history.history_steps[history_index].computed_texture,
                base_params.color(GgColor::new(1.0, 1.0, 1.0, alpha)),
            )?;

            // alphas.push(alpha);
        }

        // info!("{}", alphas.iter().map(|a| format!("{:.2}", a)).join(","));

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
