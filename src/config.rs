use std::env;
use std::sync::OnceLock;

pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub ai_api_key: String,
}

impl Config {
    fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            ai_api_key: env::var("AI_API_KEY").expect("AI_API_KEY must be set"),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3030".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid u16"),
        }
    }
}

static LOADED_CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    LOADED_CONFIG.get_or_init(|| {
        dotenvy::dotenv().ok();
        Config::from_env()
    })
}
