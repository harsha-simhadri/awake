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
        let mut best: Candidate = Candidate::default();
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

#[cfg(test)]
mod tests {
    use super::*;
    use mini_redis::client;

    #[tokio::test]
    async fn test_diskann() {
        let num_vectors = 1000;
        let dimension = 5;
        let client = client::connect("127.0.0.1:6379").await.unwrap();
        let vec_provider = MiniRedisVecProvider::new(client, "pathXYZ".to_string(), dimension);
        let mut diskann = DiskANN::new(vec_provider);
        let random_vectors = get_random_vectors(num_vectors, dimension);
        diskann.set_vectors(random_vectors.clone()).await;
        let get_result = diskann.get_vectors(&(0..num_vectors).collect()).await;

        for (i, result) in get_result.iter().enumerate() {
            assert_eq!(
                result.as_ref().unwrap(),
                &Bytes::from(random_vectors[i].clone())
            );
        }

        let query = random_vectors[0].clone();
        let top1 = diskann
            .search_simulate(query.clone(), 10, 10)
            .await
            .unwrap();
        println!("Top 1: {:?}", top1);
    }
}
