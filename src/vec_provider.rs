use bytes::Bytes;
use mini_redis::{client, Result};

pub struct VecProvider {
    client: client::Client,
    path_prefix: String,
    dimension: usize,
}

impl VecProvider {
    pub fn new(client: client::Client, path_prefix: String, dimension: usize) -> Self {
        VecProvider {
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
    let mut vec_provider = VecProvider::new(client, "pathXYZ".to_string(), 5);
    let vector: Vec<u8> = vec![1, 2, 3, 4, 5];
    vec_provider.set(1, vector.clone()).await.unwrap();
    let result = vec_provider.get(1).await.unwrap().unwrap();
    assert_eq!(result, Bytes::from(vector));
}
