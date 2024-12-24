pub mod gen;

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

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("config invalid")]
    Invalid(#[from] io::Error),
    #[error("unknown configuration error")]
    Unknown,
}

pub fn get_default_config() -> v1::Config {
    return v1::Config::default();
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
