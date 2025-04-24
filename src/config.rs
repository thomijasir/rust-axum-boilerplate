use clap::Parser;
#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum AppEnv {
    Development,
    Production,
}
#[derive(clap::Parser, Debug, Clone)]
pub struct Config {
    #[clap(long, env = "APP_ENV", default_value = "development")]
    pub app_env: AppEnv,
    #[clap(long, env = "SECRET")]
    pub secret: String,
    #[clap(long, env = "PORT", default_value = "8080")]
    pub port: u16,
    #[clap(long, env = "TIMEOUT", default_value = "300")] // 5 Minutes
    pub timeout: u64,
    #[clap(long, env = "PB_HOST", default_value = "http://localhost:8090/api")]
    pub pb_host: String,
    #[clap(
        long,
        env = "LLM_HOST",
        default_value = "https://quantune.fuelpro.io/api/v1"
    )]
    pub llm_host: String,
    #[clap(long, env = "LLM_API", default_value = "XX")]
    pub llm_api_key: String,
    #[clap(long, env = "DATABASE_URL", default_value = "")]
    pub database_url: String,
    #[clap(long, env = "MAIL_SMTP", default_value = "")]
    pub mail_smtp: String,
}
impl Config {
    pub fn load() -> Self {
        Config::parse()
    }
}
