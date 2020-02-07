use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div},
};

use ggez::{
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{self, spritebatch::SpriteBatch, DrawParam, Image, Rect},
    timer, Context, ContextBuilder, GameResult,
};
use ndarray::{s, Array2};
use rand::prelude::*;
use rayon::prelude::*;

use mutagen::{Generatable, Mutatable};

use crate::{
    constants::*,
    datatype::colors::FloatColor,
    node::{color_nodes::FloatColorNodes, Node},
    updatestate::*,
};

use ggez::event::KeyCode;
use ggez::graphics::WHITE;
use ggez::input::keyboard;

mod constants;
mod datatype;
mod node;
mod updatestate;
mod util;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("cellular3", "CodeBunny")
        .window_mode(WindowMode {
            width: INITIAL_WINDOW_WIDTH,
            height: INITIAL_WINDOW_HEIGHT,
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
    //Game draw texture
    image: Image,
    //Screen bounds
    bounds: Rect,
    //The actual cell array
    old_cell_array: Array2<FloatColor>,
    cell_array: Array2<FloatColor>,
    new_cell_array: Array2<FloatColor>,

    //rule_sets: [RuleSet; MAX_COLORS],

    //The rolling total used to calculate the average per update instead of per slice
    //rolling_update_stat_total: UpdateStat,
    //The average update stat over time, calculated by averaging rolling total and itself once an update
    //average_update_stat: UpdateStat,
    //The mechanism responsible for creating an initial state if all automata have died
    //reseeder: Reseeder,
    //The root node for the tree that computes the next screen state
    root_node: Box<FloatColorNodes>,

    tree_dirty: bool,

    current_sync_tic: i32,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let (pixels_x, pixels_y) = ggez::graphics::size(ctx);

        let cells_x = CELL_ARRAY_WIDTH;
        let cells_y = CELL_ARRAY_HEIGHT;

        MyGame {
            // ...
            image: Image::solid(ctx, 1, WHITE.into()).unwrap(),

            bounds: Rect::new(0.0, 0.0, pixels_x, pixels_y),

            old_cell_array: Array2::from_shape_fn((cells_x, cells_y), |(_x, _y)| -> FloatColor {
                FloatColor {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }
            }),
            cell_array: Array2::from_shape_fn((cells_x, cells_y), |(_x, _y)| -> FloatColor {
                FloatColor {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }
            }),
            new_cell_array: Array2::from_shape_fn((cells_x, cells_y), |(_x, _y)| -> FloatColor {
                FloatColor {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }
            }),

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

            // rolling_update_stat_total: UpdateStat {
            //     active_cells: 0,
            //     similar_neighbours: 0,
            // },
            // average_update_stat: UpdateStat {
            //     active_cells: 0,
            //     similar_neighbours: 0,
            // },

            // reseeder: Reseeder::Modulus {
            //     x_mod: 4,
            //     y_mod: 4,
            //     x_offset: random::<usize>() % CELL_ARRAY_WIDTH,
            //     y_offset: random::<usize>() % CELL_ARRAY_HEIGHT,
            //     color_table: Array2::from_shape_fn((2, 2), |_| get_random_color()),
            // },
            root_node: Box::new(
                FloatColorNodes::generate(),
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

            current_sync_tic: 0,
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
fn lerp_float_color(a: FloatColor, b: FloatColor, value: f32) -> FloatColor {
    FloatColor {
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
        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            self.tree_dirty = true;
        }

        let width = self.cell_array.dim().0 as i32;
        let height = self.cell_array.dim().1 as i32;

        let slice_height = height / TICS_PER_UPDATE;
        let slice_y = (timer::ticks(ctx) as i32 % TICS_PER_UPDATE) * slice_height;

        let slice_information = s![0..width, slice_y..slice_y + slice_height];

        let current_update_slice = self.cell_array.slice(slice_information);
        let new_update_slice = self.new_cell_array.slice_mut(slice_information);

        //let rule_sets = self.rule_sets;
        let cell_array_view = self.cell_array.view();

        let current_sync_tic = self.current_sync_tic;

        let root_node = &self.root_node;

        let _slice_update_stat: UpdateStat = ndarray::Zip::indexed(current_update_slice)
            .and(new_update_slice)
            .into_par_iter()
            .map(|((x, y), current, new)| {
                // let neighbour_result =
                //     get_alive_neighbours(cell_array_view, x as i32, y as i32 + slice_y);

                let new_color = root_node.compute(UpdateState {
                    coordinate_set: CoordinateSet {
                        x: x as f32,
                        y: (y as f32 + slice_y as f32),
                        t: current_sync_tic as f32,
                    },
                    cell_array: cell_array_view,
                }); //get_next_color(rule_sets, *current, neighbour_result.0);

                let older_color = *new;
                *new = new_color;

                UpdateStat {
                    //Two checks are necessary to avoid two tic oscillators being counted as active cells
                    active_cells: if new_color != older_color && new_color != *current {
                        1
                    } else {
                        0
                    },
                    similar_neighbours: 0, //neighbour_result.1,
                }
            })
            .sum();

        //self.rolling_update_stat_total += slice_update_stat;

        let _total_cells = width * height;

        if timer::ticks(ctx) as i32 % TICS_PER_UPDATE == 0 {
            self.current_sync_tic += 1;

            //self.average_update_stat =
            //    (self.average_update_stat + self.rolling_update_stat_total) / 2;

            // let sqrt_stagnant_cells =
            //     ((total_cells - slice_update_stat.active_cells) as f32).sqrt() as i32;

            // let activity_value = self.average_update_stat.active_cells as f32 / total_cells as f32;
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

            // self.rolling_update_stat_total = UpdateStat {
            //     active_cells: 0,
            //     similar_neighbours: 0,
            // };

            if self.tree_dirty || random::<u32>() % 100 == 0 {
                self.root_node.mutate();
                println!("====MUTATING TREE====");
                println!("{:#?}", &self.root_node);
                self.tree_dirty = false;
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

                let lerped_color = lerp_float_color(*old, *current, lerp_value);

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
