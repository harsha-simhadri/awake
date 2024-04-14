pub trait VecProvider {
    async fn get(&mut self, vec_id: usize) -> Result<Option<Bytes>>;
    async fn set(&mut self, vec_id: usize, value: Vec<u8>) -> Result<()>;
}