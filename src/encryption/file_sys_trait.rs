use std::path::PathBuf;

pub trait FileSystem {
    fn new(path: PathBuf) -> Self;
    fn create_file(
        &self,
    ) -> impl std::future::Future<Output = Result<std::fs::File, std::io::Error>> + Send;
    fn read_file(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, std::io::Error>> + Send;
    fn encrypt_write_file(
        &self,
        data: Vec<u8>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
    fn write_file(&self, data: Vec<u8>) -> anyhow::Result<()>;

    fn encrypt_file(&self) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;

    fn decrypt_read_file(
        &self,
    ) -> impl std::future::Future<Output = anyhow::Result<Vec<u8>>> + Send;
    fn decrypt_file(&self) -> impl std::future::Future<Output = anyhow::Result<()>> + Send;
}
