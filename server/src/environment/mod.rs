use std::env;

pub struct Env;

impl Env {
    pub fn jwt_secret() -> String {
        env::var("JWT_SECRET").expect("JWT_SECRET must be set")
    }

    pub fn database_url() -> String {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    }

    pub fn client_url() -> String {
        env::var("CLIENT_URL").expect("CLIENT_URL must be set")
    }
}
