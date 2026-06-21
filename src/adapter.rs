#[derive(Clone)]
pub struct ToastyAdapter {
    pub db: toasty::Db,
}

impl ToastyAdapter {
    pub fn new(db: toasty::Db) -> Self {
        Self { db }
    }
}
