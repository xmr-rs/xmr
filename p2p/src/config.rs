use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Config {
    pub threads: usize,
    pub network_id: Uuid,
}
