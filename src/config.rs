use std::fs;
use yaml_rust::YamlLoader;

#[derive(Debug, Default)]
pub struct Config {
    pub crawlio_key: String,
    pub surreal_address: String,
    pub surreal_username: String,
    pub surreal_password: String,
    pub surreal_namespace: String,
    pub surreal_database: String,
}

pub fn load_config() -> Result<Config, String> {
    let key_file = match fs::read_to_string("config.yaml") {
        Err(e) => panic!("Problem opening config file: {:?}", e),
        Ok(f) => f,
    };

    let docs = YamlLoader::load_from_str(&*key_file).unwrap();
    if docs.is_empty() {
        panic!("Key file is empty");
    }

    let mut config_found = Config::default();

    let doc = &docs[0];

    config_found.crawlio_key = match doc["crawloi_key"].as_str() {
        Some(key) => key.to_string(),
        None => return Err("Can't find Crawlio key in config file.".to_string()),
    };

    config_found.surreal_address = match doc["surreal_address"].as_str() {
        Some(address) => address.to_string(),
        None => return Err("Can't find SurrealDB server address in config file.".to_string()),
    };

    config_found.surreal_username = match doc["surreal_username"].as_str() {
        Some(username) => username.to_string(),
        None => return Err("Can't find SurrealDB server username in config file.".to_string()),
    };

    config_found.surreal_password = match doc["surreal_password"].as_str() {
        Some(password) => password.to_string(),
        None => return Err("Can't find SurrealDB server password in config file.".to_string()),
    };

    config_found.surreal_namespace = match doc["surreal_namespace"].as_str() {
        Some(namespace) => namespace.to_string(),
        None => return Err("Can't find SurrealDB server namespace in config file.".to_string()),
    };

    config_found.surreal_database = match doc["surreal_database"].as_str() {
        Some(database) => database.to_string(),
        None => return Err("Can't find SurrealDB server database in config file.".to_string()),
    };

    Ok(config_found)
}
