use {
    gen::dev::vsx::svctl::v1,
    prost::Message,
    std::{
        fs::File,
        io::{self, BufRead, BufReader},
        path::Path,
    },
    thiserror::Error,
};

pub mod gen;
pub use gen::dev;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("config invalid")]
    Invalid(#[from] io::Error),
    #[error("unknown configuration error")]
    Unknown,
}

pub fn get_default_config() -> v1::Config {
    let mut config = v1::Config::default();

    config.user_name = String::from("sol");
    config.home_path = String::from(format!("/home/{0}", config.user_name));
    config.source_path = String::from("src/github.com");
    config.clusters = vec![];
    config.machines = vec![];

    config
}

pub fn read_config(path: &Path) -> Result<v1::Config, ConfigError> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    // let capacity = reader.capacity();
    let buffer = reader.fill_buf()?;
    let mut config = get_default_config();
    let result = v1::Config::merge_length_delimited(&mut config, buffer);
    result.unwrap_or_else(|err| {
        panic!("invalid config: {err}");
    });
    return Ok(config);
}
