use rand::prelude::*;

#[derive(Debug)]
pub struct Dice {
    faces: u32,
    number_of_dice: u32,
    roll_result: Vec<u32>,
    rng: ThreadRng,
}

impl Dice {
    fn new(faces: u32, number_of_dice: u32) -> Dice {
        Dice {
            faces,
            number_of_dice,
            roll_result: Vec::new(),
            rng: thread_rng(),
        }
    }

    pub fn roll(faces: u32, number_of_dice: u32, times: u32) {
        let mut dice = Dice::new(faces, number_of_dice);
        for _ in 0..times {
            dice.generate_randoms();
            dice.display_result();
            dice.roll_result.clear();
        }
    }

    fn generate_randoms(&mut self) {
        for _x in 0..self.number_of_dice {
            self.roll_result.push(self.rng.gen_range(1, self.faces + 1))
        }
    }

    fn display_result(&self) {
        println!("{:?}", self.roll_result)
    }
}

