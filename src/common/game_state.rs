extern crate rand;

use self::rand::{Rng, SeedableRng};
use std::mem;
use stdweb::web::Date;

use inner_common::*;

use vm::VM;

impl GameState {
    pub fn new() -> GameState {
        let seed = unsafe {
            let time = Date::new().get_time();

            mem::transmute::<[f64; 2], [u32; 4]>([time, 1.0 / time])
        };

        console!(log, format!("{:?}", seed));
        let mut rng = rand::XorShiftRng::from_seed(seed);

        let mut turtles = [(0, 0); TURTLE_COUNT];
        let mut instructions = [[0; INSTRUCTION_PAIR_COUNT]; TURTLE_COUNT];

        for i in 0..TURTLE_COUNT {
            turtles[i] = rng.gen();
            instructions[i] = rng.gen();
        }

        GameState {
            turtles,
            instructions,
        }
    }
}
