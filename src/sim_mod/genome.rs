use rand::{random, random_bool};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use crate::constants::simulation::MUTATION_RATE;

const GENOME_SIZE: usize = 1000;

#[derive(Default)]
pub struct Genome {
    data: Vec<u16>,
    value_counter: usize,
}

impl Genome {
    pub fn new() -> Genome {
        let data: Vec<u16> = (0..GENOME_SIZE).into_par_iter().map(|_| random()).collect();
        Genome { data, ..Default::default() }
    }

    fn parse_values(&mut self, amount: usize) -> Vec<u16> {
        let mut values = Vec::<u16>::with_capacity(amount);
        // check if we need to wrap around
        if (self.value_counter + amount) > GENOME_SIZE {
            values = (self.value_counter..GENOME_SIZE)
                .into_iter().chain(
                0..(amount - (GENOME_SIZE - self.value_counter)))
                .map(|x| self.data[x])
                .collect();
        } else {
            values = (self.value_counter..self.value_counter + amount)
                .into_iter()
                .map(|x| self.data[x])
                .collect();
        }
        self.value_counter =(self.value_counter + amount) % GENOME_SIZE;
        values
    }

    pub fn parse_value(&mut self) -> u16 {
        let out = self.data[self.value_counter];
        self.value_counter = self.value_counter + 1;
        out
    }
    pub fn parse_value_normalized(&mut self) -> f32 {
        let out = self.data[self.value_counter];
        self.value_counter = self.value_counter + 1;
        out as f32 / u16::MAX as f32
    }

    pub fn parse_bool(&mut self) -> bool {
        let out = self.parse_value_normalized();
        out >= 0.5
    }

    pub fn create_offspring(&self) -> Genome {
        let data = self.data.par_iter().map(|x| Self::mutate(&x)).collect();
        Genome { data, ..Default::default() }
    }

    fn mutate(val: &u16) -> u16 {
        if random_bool(MUTATION_RATE) {
            random()
        } else {
            *val
        }
    }
}