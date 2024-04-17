use bytes::Bytes;
use mini_redis::Error;

pub trait VecProvider {
    fn get_zero(&mut self) -> impl std::future::Future<Output = i32> + Send;
    fn get(
        &mut self,
        vec_id: usize,
    ) -> impl std::future::Future<Output = Result<Option<Bytes>, Error>>;
    fn set(
        &mut self,
        vec_id: usize,
        value: Vec<u8>,
    ) -> impl std::future::Future<Output = Result<(), Error>>;
}
