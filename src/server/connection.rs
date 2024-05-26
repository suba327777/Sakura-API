use crate::domain::repository::account::AccountRepository;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub struct RequestContext {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl RequestContext {
    pub fn new() -> RequestContext {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL i not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create DB connection pool.");

        RequestContext { pool }
    }

    pub fn account_repository(&self) -> impl AccountRepository {
        use crate::infrastructures::repository::account::AccountRepositoryImpl;

        AccountRepositoryImpl {
            pool: Box::new(self.pool.to_owned()),
        }
    }
}
