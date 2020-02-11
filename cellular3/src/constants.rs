pub const TICS_PER_UPDATE: usize = 6;

pub const INITIAL_WINDOW_WIDTH: f32 = 1600.0;
pub const INITIAL_WINDOW_HEIGHT: f32 = 900.0;
pub const VSYNC: bool = false;

pub const CELL_ARRAY_WIDTH: usize = 400;
pub const CELL_ARRAY_HEIGHT: usize = 225;
pub const CELL_ARRAY_HISTORY_LENGTH: usize = 4;

pub const NOISE_X_SCALE_FACTOR: f64 = 4.0;
pub const NOISE_Y_SCALE_FACTOR: f64 = 4.0;
pub const NOISE_T_SCALE_FACTOR: f64 = 0.5;
pub const _NOISE_X_SCALE_MINIMUM: f64 = 0.001;
pub const _NOISE_Y_SCALE_MINIMUM: f64 = 0.001;
pub const _NOISE_T_SCALE_MINIMUM: f64 = 0.5;

pub const IMAGE_PATH: &str =
    "C:\\Users\\admin\\Documents\\Project Assets\\Cellular\\Images\\!WorkAppropriate";

//primitive consts
pub const BYTE_MAX_VALUE: u64 = 255;
pub const BYTE_POSSIBLE_VALUES: u64 = 256;

//neighbour consts
pub const _MAX_NEIGHBOUR_ARRAY_COUNT: usize = 9; //Use this for array indexes as it counts zero
pub const _MAX_NEIGHBOUR_COUNT: i32 = 8; //Use this for total neighbours excluding zero

//color consts
pub const MAX_COLORS: usize = 8;

pub const PARALLELIZE: bool = false;

pub const MIN_LEAF_DEPTH: usize = 1;
pub const MAX_LEAF_DEPTH: usize = 1000;

pub const MIN_PIPE_DEPTH: usize = 1;
pub const MAX_PIPE_DEPTH: usize = 10;

pub const MIN_BRANCH_DEPTH: usize = 0;
pub const MAX_BRANCH_DEPTH: usize = 5;
