use rand::prelude::*;

pub fn get_random_vectors(num_vectors: usize, dimension: usize) -> Vec<Vec<u8>> {
    (0..num_vectors)
        .map(|_| (0..dimension).map(|_| random::<u8>()).collect())
        .collect()
}

pub fn l2_distance(v1: &[u8], v2: &[u8]) -> f32 {
    v1.iter()
        .zip(v2.iter())
        .map(|(a, b)| (*a as f32 - *b as f32).powi(2))
        .sum::<f32>()
        .sqrt()
}

#[derive(Debug)]
pub struct Candidate {
    id: usize,
    distance: f32,
}

impl Candidate {
    pub fn new() -> Self {
        Candidate {
            id: 0,
            distance: std::f32::MAX,
        }
    }

    pub fn update(&mut self, id: usize, distance: f32) {
        if distance < self.distance {
            self.id = id;
            self.distance = distance;
        }
    }
}
