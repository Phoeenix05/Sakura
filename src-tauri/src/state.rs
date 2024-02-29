use surrealdb::{engine::local::Db, Surreal};

pub struct SakuraState {
    db: Surreal<Db>,
}

impl SakuraState {
    pub fn init() -> Self {
        let db = Surreal::init();

        Self { db }
    }
}
