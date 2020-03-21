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
        points::*,
    },
    node::{
        color_nodes::*, continuous_nodes::*, discrete_nodes::*, point_nodes::*, Node,
    },
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

    rotation: f32,
    translation: SNPoint,
    offset: SNPoint,
    from_scale: SNPoint,
    to_scale: SNPoint,

    apply_rotation: bool,
    apply_translation: bool,
    apply_offset: bool,
    apply_scale: bool,
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
                    rotation: 0.0,
                    translation: SNPoint::zero(),
                    offset: SNPoint::zero(),
                    from_scale: SNPoint::zero(),
                    to_scale: SNPoint::zero(),
                    apply_rotation: false,
                    apply_translation: false,
                    apply_offset: false,
                    apply_scale: false,
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
            a: raw[3],
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
    root_angle_node: Box<SNFloatNodes>,
    root_translation_node: Box<SNPointNodes>,
    root_offset_node: Box<SNPointNodes>,
    root_from_scale_node: Box<SNPointNodes>,
    root_to_scale_node: Box<SNPointNodes>,

    apply_angle_node: Box<BooleanNodes>,
    apply_translation_node: Box<BooleanNodes>,
    apply_offset_node: Box<BooleanNodes>,
    apply_scale_node: Box<BooleanNodes>,

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
                rotation: 0.0,
                translation: SNPoint::zero(),
                offset: SNPoint::zero(),
                from_scale: SNPoint::zero(),
                to_scale: SNPoint::zero(),
                apply_rotation: false,
                apply_translation: false,
                apply_offset: false,
                apply_scale: false,
            },
            history: History::new(
                ctx,
                CONSTS.cell_array_width,
                CONSTS.cell_array_height,
                CONSTS.cell_array_history_length,
            ),
            rolling_update_stat_total: UpdateStat {
                activity_value: 0.0,
                alpha_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
            },
            average_update_stat: UpdateStat {
                activity_value: 0.0,
                alpha_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
            },

            // root_node: Box::new(FloatColorNodes::HSVMandelbrot{
            //     child_power: Box::new(UNFloatNodes::generate_rng(
            //         &mut rng,
            //         mutagen::State::default(),
            //     )),
            //     child_offset: Box::new(SNPointNodes::generate_rng(
            //         &mut rng,
            //         mutagen::State::default(),
            //     ))
            // }),

            root_node: Box::new(FloatColorNodes::generate_rng(
                &mut rng,
                mutagen::State::default(),
            )),

            root_angle_node: Box::new(SNFloatNodes::generate_rng(
                &mut rng,
                mutagen::State::default(),
            )),
            root_translation_node: Box::new(SNPointNodes::generate_rng(
                &mut rng,
                mutagen::State::default(),
            )),
            root_offset_node: Box::new(SNPointNodes::generate_rng(
                &mut rng,
                mutagen::State::default(),
            )),
            root_from_scale_node: Box::new(SNPointNodes::generate_rng(
                &mut rng,
                mutagen::State::default(),
            )),
            root_to_scale_node: Box::new(SNPointNodes::generate_rng(
                &mut rng,
                mutagen::State::default(),
            )),

            apply_angle_node: Box::new(BooleanNodes::generate_rng(
                &mut rng,
                mutagen::State::default(),
            )),
            apply_translation_node: Box::new(BooleanNodes::generate_rng(
                &mut rng,
                mutagen::State::default(),
            )),
            apply_offset_node: Box::new(BooleanNodes::generate_rng(
                &mut rng,
                mutagen::State::default(),
            )),
            apply_scale_node: Box::new(BooleanNodes::generate_rng(
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
    alpha_value: f32,
    local_similarity_value: f32,
    global_similarity_value: f32,
}

impl Add<UpdateStat> for UpdateStat {
    type Output = UpdateStat;

    fn add(self, other: UpdateStat) -> UpdateStat {
        UpdateStat {
            activity_value: self.activity_value + other.activity_value,
            alpha_value: self.alpha_value + other.alpha_value,
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
            alpha_value: self.alpha_value / other,
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

fn lerp(a: f32, b: f32, value: f32) -> f32 {
    a + (b - a) * value
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
            new[3] = new_color.a;

            let current_color = history.get(x, y, current_t);
            let older_color = history.get(x, y, usize::max(current_t, 1) - 1);

            let local_offset = (thread_rng().gen_range(-1, 2), thread_rng().gen_range(-1, 2));
            let local_color = history.get(
                (x as i32 + local_offset.0)
                    .max(0)
                    .min(CONSTS.cell_array_width as i32 - 1) as usize,
                (y as i32 + local_offset.1).min(CONSTS.cell_array_height as i32 - 1) as usize,
                current_t,
            );
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
                alpha_value: (current_color.a as f32 / 256.0) / total_cells as f32,
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
                alpha_value: 0.0,
                local_similarity_value: 0.0,
                global_similarity_value: 0.0,
            };

            if self.tree_dirty
                || dbg!(f64::from(self.average_update_stat.activity_value)) < CONSTS.activity_value_lower_bound
                || dbg!(f64::from(self.average_update_stat.alpha_value)) < CONSTS.alpha_value_lower_bound
                || dbg!(f64::from(self.average_update_stat.local_similarity_value)) > CONSTS.local_similarity_upper_bound
                || dbg!(f64::from(self.average_update_stat.global_similarity_value)) >= CONSTS.global_similarity_upper_bound
                // || self.average_update_stat.activity_value > 0.5
            {
                info!("====TIC: {} MUTATING TREE====", self.current_t);
                self.root_node
                    .mutate_rng(&mut self.rng, mutagen::State::default());
                info!("{:#?}", &self.root_node);

                match thread_rng().gen::<usize>() % 9 {
                    0 => {
                        self.root_angle_node
                            .mutate_rng(&mut self.rng, mutagen::State::default());
                        info!("{:#?}", &self.root_angle_node);
                    }
                    1 => {
                        self.root_translation_node
                            .mutate_rng(&mut self.rng, mutagen::State::default());
                        info!("{:#?}", &self.root_translation_node);
                    }
                    2 => {
                        self.root_offset_node
                            .mutate_rng(&mut self.rng, mutagen::State::default());
                        info!("{:#?}", &self.root_offset_node);
                    }
                    3 => {
                        self.root_from_scale_node
                            .mutate_rng(&mut self.rng, mutagen::State::default());
                        info!("{:#?}", &self.root_from_scale_node);
                    }
                    4 => {
                        self.root_to_scale_node
                            .mutate_rng(&mut self.rng, mutagen::State::default());
                        info!("{:#?}", &self.root_to_scale_node);
                    }
                    5 => {
                        self.apply_angle_node
                            .mutate_rng(&mut self.rng, mutagen::State::default());
                        info!("{:#?}", &self.apply_angle_node);
                    }
                    6 => {
                        self.apply_translation_node
                            .mutate_rng(&mut self.rng, mutagen::State::default());
                        info!("{:#?}", &self.apply_translation_node);
                    }
                    7 => {
                        self.apply_offset_node
                            .mutate_rng(&mut self.rng, mutagen::State::default());
                        info!("{:#?}", &self.apply_offset_node);
                    }
                    8 => {
                        self.apply_scale_node
                            .mutate_rng(&mut self.rng, mutagen::State::default());
                        info!("{:#?}", &self.apply_scale_node);
                    }
                    _ => {
                        panic!();
                    }
                }
                self.tree_dirty = false;
            }

            self.next_history_step.rotation = self
                .root_angle_node
                .compute(UpdateState {
                    coordinate_set: CoordinateSet {
                        x: SNFloat::new(0.0),
                        y: SNFloat::new(0.0),
                        t: self.current_t as f32,
                    },
                    history: &self.history,
                })
                .into_inner();
            self.next_history_step.translation = self.root_translation_node.compute(UpdateState {
                coordinate_set: CoordinateSet {
                    x: SNFloat::new(0.0),
                    y: SNFloat::new(0.0),
                    t: self.current_t as f32,
                },
                history: &self.history,
            });
            self.next_history_step.offset = self.root_offset_node.compute(UpdateState {
                coordinate_set: CoordinateSet {
                    x: SNFloat::new(0.0),
                    y: SNFloat::new(0.0),
                    t: self.current_t as f32,
                },
                history: &self.history,
            });
            self.next_history_step.from_scale = self.root_from_scale_node.compute(UpdateState {
                coordinate_set: CoordinateSet {
                    x: SNFloat::new(0.0),
                    y: SNFloat::new(0.0),
                    t: self.current_t as f32,
                },
                history: &self.history,
            });
            self.next_history_step.to_scale = self.root_to_scale_node.compute(UpdateState {
                coordinate_set: CoordinateSet {
                    x: SNFloat::new(0.0),
                    y: SNFloat::new(0.0),
                    t: self.current_t as f32,
                },
                history: &self.history,
            });

            self.next_history_step.apply_rotation = self
                .apply_angle_node
                .compute(UpdateState {
                    coordinate_set: CoordinateSet {
                        x: SNFloat::new(0.0),
                        y: SNFloat::new(0.0),
                        t: self.current_t as f32,
                    },
                    history: &self.history,
                })
                .into_inner();
            self.next_history_step.apply_translation = self
                .apply_translation_node
                .compute(UpdateState {
                    coordinate_set: CoordinateSet {
                        x: SNFloat::new(0.0),
                        y: SNFloat::new(0.0),
                        t: self.current_t as f32,
                    },
                    history: &self.history,
                })
                .into_inner();
            self.next_history_step.apply_offset = self
                .apply_offset_node
                .compute(UpdateState {
                    coordinate_set: CoordinateSet {
                        x: SNFloat::new(0.0),
                        y: SNFloat::new(0.0),
                        t: self.current_t as f32,
                    },
                    history: &self.history,
                })
                .into_inner();
            self.next_history_step.apply_scale = self
                .apply_scale_node
                .compute(UpdateState {
                    coordinate_set: CoordinateSet {
                        x: SNFloat::new(0.0),
                        y: SNFloat::new(0.0),
                        t: self.current_t as f32,
                    },
                    history: &self.history,
                })
                .into_inner();

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
        graphics::clear(ctx, graphics::BLACK);

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
            let alpha = 
            1.0 - 
            ((i as f32 - lerp_value) / (lerp_len - 1) as f32).max(0.0)
                    //.powf(CONSTS.lerp_aggressiveness)
                    ;

            let hist_len = self.history.history_steps.len();
            let history_index = (self.current_t + i + hist_len - lerp_len) % hist_len;
            let history_step = &self.history.history_steps[history_index];

            let dest_offset_x = if history_step.apply_translation {
                (CONSTS.initial_window_width
                    * history_step.translation.into_inner().x
                    * 0.5
                    * (1.0 - alpha))
            } else {
                0.0
            };
            let dest_offset_y = if history_step.apply_translation {
                (CONSTS.initial_window_height
                    * history_step.translation.into_inner().y
                    * 0.5
                    * (1.0 - alpha))
            } else {
                0.0
            };

            let offset_offset_x = if history_step.apply_offset {
                (history_step.offset.into_inner().x * 0.5 * (1.0 - alpha))
            } else {
                0.0
            };
            let offset_offset_y = if history_step.apply_offset {
                (history_step.offset.into_inner().y * 0.5 * (1.0 - alpha))
            } else {
                0.0
            };

            let x_scale_ratio = CONSTS.initial_window_width / CONSTS.cell_array_width as f32;
            let y_scale_ratio = CONSTS.initial_window_height / CONSTS.cell_array_height as f32;

            let scale_x = if history_step.apply_scale {
                lerp(1.0 + history_step.from_scale.into_inner().x, 1.0 + history_step.to_scale.into_inner().x, alpha)
            } else {
                0.0
            };
            let scale_y = if history_step.apply_scale {
                lerp(1.0 + history_step.from_scale.into_inner().y, 1.0 + history_step.to_scale.into_inner().y, alpha)
            } else {
                0.0
            };

            let rotation = if history_step.apply_rotation {
                (1.0 - alpha) * history_step.rotation * 3.14
            } else {
                0.0
            };

            ggez::graphics::draw(
                ctx,
                &history_step.computed_texture,
                base_params
                    .color(GgColor::new(1.0, 1.0, 1.0, 1.0 - ((alpha * 2.0) - 1.0).abs()))
                    .dest([
                        ((CONSTS.initial_window_width * 0.5) + dest_offset_x * scale_x),
                        ((CONSTS.initial_window_height * 0.5) + dest_offset_y * scale_y),
                    ])
                    // .offset([0.5 + offset_offset_x, 0.5 + offset_offset_y])
                    .offset([0.5, 0.5])
                    .scale([(1.0 + scale_x) * x_scale_ratio, (1.0 + scale_y) * y_scale_ratio])
                    .rotation(rotation),
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
