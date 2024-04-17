use crate::miniredis_vec_provider::*;
use mini_redis::{client, Result};

pub mod c_vector_provider;
pub mod diskann;
pub mod ffi_async_test;
pub mod miniredis_vec_provider;
pub mod utils;
pub mod vec_provider_trait;

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
