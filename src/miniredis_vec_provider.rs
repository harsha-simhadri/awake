use bytes::Bytes;
use mini_redis::{client, Result};
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

// pub struct GetFuture<'a> {
//     client: &'a mut client::Client,
//     result: &'a Box<dyn Future<Output = Result<Option<Bytes>>>,// Box<dyn Err + Send + Sync, Global>>>,
// }

// impl<'a> Future for GetFuture<'a> {
//     type Output = Result<Option<Bytes>>;
//     fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
//         self.result.as_mut().poll(cx)
//     }
// }

pub struct MiniRedisVecProvider {
    client: client::Client,
    path_prefix: String,
    dimension: usize,
}

impl MiniRedisVecProvider {
    pub fn new(client: client::Client, path_prefix: String, dimension: usize) -> Self {
        MiniRedisVecProvider {
            client,
            path_prefix,
            dimension,
        }
    }

    pub async fn get(&mut self, vec_id: usize) -> Result<Option<Bytes>> {
        let key = format!("{}:{}", self.path_prefix, vec_id);
        self.client.get(&key).await
    }

    pub async fn set(&mut self, vec_id: usize, value: Vec<u8>) -> Result<()> {
        if self.dimension != value.len() {
            return Err("Dimension mismatch".into());
        }
        let key = format!("{}:{}", self.path_prefix, vec_id);
        self.client.set(&key, Bytes::from(value)).await
    }
}

#[tokio::test]
async fn test_set_get() {
    let client = client::connect("127.0.0.1:6379").await.unwrap();
    let mut vec_provider = MiniRedisVecProvider::new(client, "pathXYZ".to_string(), 5);
    let vector: Vec<u8> = vec![1, 2, 3, 4, 5];
    vec_provider.set(1, vector.clone()).await.unwrap();
    let result = vec_provider.get(1).await.unwrap().unwrap();
    assert_eq!(result, Bytes::from(vector));
}
