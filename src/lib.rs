use rand::prelude::*;
use std::fmt;

#[derive(Debug)]
pub struct Dice {
    faces: u32,
    number_of_dice: u32,
    roll_result: Vec<u32>,
    rng: ThreadRng,
}

impl Dice {
    pub fn new(faces: u32, number_of_dice: u32) -> Dice {
        Dice {
            faces,
            number_of_dice,
            roll_result: Vec::new(),
            rng: thread_rng(),
        }
    }

    pub fn roll(&mut self) {
        for _x in 0..self.number_of_dice {
            self.roll_result.push(self.rng.gen_range(1, self.faces + 1))
        }
    }

    pub fn display_result(&self) {
        println!("{:#?}", self.roll_result)
    }
}

