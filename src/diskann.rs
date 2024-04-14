use bytes::Bytes;
//use futures::future::{join_all, try_join_all};
use mini_redis::Result;
use rand::prelude::*;
use std::sync::atomic::AtomicUsize;

use crate::utils::*;
use crate::MiniRedisVecProvider;

pub struct DiskANN {
    vec_provider: MiniRedisVecProvider,
    num_vectors: AtomicUsize,
}

impl DiskANN {
    pub fn new(vector_provider: MiniRedisVecProvider) -> Self {
        DiskANN {
            vec_provider: vector_provider,
            num_vectors: AtomicUsize::new(0),
        }
    }

    pub async fn set_vectors(&mut self, vectors: Vec<Vec<u8>>) {
        for (i, vector) in vectors.into_iter().enumerate() {
            self.vec_provider.set(i, vector).await.unwrap();
            self.num_vectors
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    pub async fn get_vectors(&mut self, ids: &Vec<usize>) -> Vec<Option<Bytes>> {
        let mut result = vec![];
        for id in ids.iter() {
            result.push(self.vec_provider.get(*id).await.unwrap());
        }
        result
    }

    /// Get top 1
    /// Random `rounds` of beam width gets
    pub async fn search_simulate(
        &mut self,
        query: Vec<u8>,
        rounds: usize,
        beamwidth: usize,
    ) -> Result<Candidate> {
        let mut best: Candidate = Candidate::new();
        for _ in 0..rounds {
            let rand_ids = (0..beamwidth)
                .map(|_| {
                    random::<usize>() % self.num_vectors.load(std::sync::atomic::Ordering::SeqCst)
                })
                .collect();
            let result = self.get_vectors(&rand_ids).await;
            for (i, bytes) in result.into_iter().enumerate() {
                let distance = l2_distance(&query, &bytes.unwrap());
                best.update(rand_ids[i], distance);
            }
        }

        Ok(best)
    }
}
