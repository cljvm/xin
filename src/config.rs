use failure::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub db: DbConfig,
    pub email: EmailConfig,
}

#[derive(Deserialize)]
pub struct DbConfig {
    protocal: String,
    user: String,
    password: String,
    host: String,
    port: u16,
    database_name: String,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Deserialize)]
pub struct EmailConfig {
    user: String,
    password: String,
    nick_name: String,
}

impl Config {
    pub fn parse(content: &str) -> Result<Config, Error> {
        let config: Config = match toml::from_str(content) {
            Ok(c) => c,
            Err(e) => bail!(e),
        };

        Ok(config)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config, Error> {
        let mut content = String::new();
        // let path = path.as_ref();
        // let file_name = path.file_name().unwrap();
        File::open(path)?
        // .chain_err(|| format_err!("配置文件：{:?}不存在！", file_name))?
        .read_to_string(&mut content)?;

        Config::parse(&content)
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

impl fmt::Display for DbConfig {
    // postgres://user:password@localhost:5432/lemondb
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}://{}:{}@{}:{}/{}",
            self.protocal, self.user, self.password, self.host, self.port, self.database_name
        )
    }
}
