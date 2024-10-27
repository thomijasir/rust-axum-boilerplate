use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum RustEnv {
  Development,
  Production,
}

#[derive(clap::Parser, Debug)]
pub struct AppConfig {
  #[clap(long, env = "RUST_ENV", default_value = "development")]
  pub rust_env: RustEnv,
  #[clap(long, env = "SECRET")]
  pub secret: String,
  #[clap(long, env = "PORT", default_value = "8080")]
  pub port: u16,
  #[clap(long, env = "TIMEOUT", default_value = "8080")]
  pub timeout: u64,
}

impl AppConfig {
  pub fn load() -> Self {
    AppConfig::parse()
  }
}
