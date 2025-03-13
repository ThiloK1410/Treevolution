use macroquad::math::IVec2;

pub const GRID_SIZE: IVec2 = IVec2::new(512, 32);
pub const ROOT_CON_DECAY: f32 = 0.8;        // how fast the connection to the root decays over distance

pub const TRUNK_ABSORB_RATE: f32 = 0.1;     // how much sun gets blocked by trunks

pub const LEAF_ABSORB_RATE: f32 = 0.25;      // how much sun gets absorbed by leaves

pub const MUTATION_RATE: f64 = 0.005;        // the probability that a genome value mutates

pub const GENOME_SIZE: usize = 500;        // the size of genome raw data, unused data is intended
                                            // -> changing this should have little to no effect

pub const RESPONSE_SIZE: usize = 30;       // the amount of response clusters a genome can hold
                                            // -> a response cluster holds all 4 responses for a cell
pub const HEIGHT_THRESHOLD_CHANCE: f32 = 0.5;// chance that a Response is locked behind a height threshold

pub const CELL_GROWTH_COST: f32 = 1.0;      //

pub const MAX_GROWTHS_PER_ITERATION: usize = 1; // each iteration each plant can grow this amount of cells at max

pub const CELL_SUSTAIN_ENERGY_COST: f32 = 0.5;

pub const DEFAULT_ENERGY: f32 = 125.;       // the default energy each plant starts with

pub const SUN_POWER: f32 = 4.;

pub const SEED_ENERGY_DRAIN: f32 = 0.5;      // the amount of energy each seed looses while staying dormant in the ground

pub const DEAD_CELL_REMOVE_RATE: f32 = 0.15; // the chance that a dead cell disappears

pub const SEED_SPAWN_RATE: f32 = 0.1;      // temporary: chance at which a dead cell becomes a seed

pub const BASE_MAX_AGE: usize = 40;         // the default max age of a plant

pub const MAX_AGE_CELL_MODIFIER: usize = 3; // the amount each grown cell increases the lifetime of the plant

pub const LIFETIME_FACTOR: f32 = 1.;        // how much the lifetime of a plant increases the energy of seeds