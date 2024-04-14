use bytes::Bytes;
use mini_redis::{client, Result};

use crate::diskann::*;
use crate::miniredis_vec_provider::*;
use crate::utils::*;

pub mod diskann;
pub mod miniredis_vec_provider;
pub mod c_vector_provider;
pub mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);


    Ok(())
}

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
