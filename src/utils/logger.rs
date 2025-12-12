pub fn init() -> () {
    dotenvy::dotenv().ok();

    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("api=debug,info,error"),
    )
    .format_timestamp_millis()
    .init();

    log::info!("Logger initialized");
}
