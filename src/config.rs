use std::fmt;
use std::fs::File;
use std::path::Path;
use toml;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub server: ServerConfig,
    pub db: DbConfig,
    pub email: EmailConfig,
}

pub(crate) struct DbConfig {
    protocal: String,
    user: String,
    password: String,
    url: String,
    port: u16,
    database_name: String,
}

pub(crate) struct ServerConfig {
    url: String,
    port: u16,
}

pub(crate) struct EmailConfig {
    user: String,
    password: String,
    nick_name: String,
}

impl Config {
    pub fn parse(content: &str) -> Result<Config> {
        let mut config: Config = match toml::from_str(content) {
            Ok(c) => c,
            Err(e) => bail!(e),
        };

        Ok(config)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut content = String::new();
        let path = path.as_ref();
        let file_name = path.file_name().unwrap();
        File::open(path)
            .chain_err(|| format_err!("配置文件：{:?}不存在！", file_name))?
            .read_to_string(&mut content)?;

        Config::parse(&content)
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.url, self.port)
    }
}

impl fmt::Display for DbConfig {
    // postgres://user:password@localhost:5432/lemondb
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}://{}:{}@{}:{}/{}",
            self.protocal,
            self.user,
            self.password,
            self.url,
            self.port,
            self.database_name
        )
    }
}
