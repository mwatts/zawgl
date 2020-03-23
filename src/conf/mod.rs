extern crate serde;
extern crate toml;

use self::serde::{Deserialize};
use std::path;
use std::fs;
use std::io::Read;

#[derive(Deserialize)]
pub struct Conf {
    pub db_dir: String,
    pub node_store: String,
    pub relationship_store: String,
    pub properties_store: String,
}

pub fn load_conf(main_dir: &str) -> Conf {
    let mut file_path = path::PathBuf::new();
    file_path.push(main_dir);
    file_path.push("orange-db.toml");
    let mut configuration_file = fs::OpenOptions::new()
        .read(true)
        .open(file_path.as_path())
        .expect("Cannot open the configuration file");
    
    let mut content = String::new();

    match configuration_file.read_to_string(&mut content) {
        Ok(bytes) => println!("{} bytes has been appended to buffer.", bytes),
        Err(error) => panic!(
            "
            The data in this stream is not valid UTF-8.\n
            See error: '{}'
            ",
            error
        ),
    }
    toml::from_str(content.as_str()).expect("Something went wrong")
}

#[cfg(test)]
mod test_conf {
    use super::*;
    #[test]
    fn test_load_conf() {
        let cfg = load_conf(".\\dist");
        assert_eq!(cfg.db_dir, "C:\\Temp");
        assert_eq!(cfg.node_store, "nodes.db");
        assert_eq!(cfg.properties_store, "properties.db");
        assert_eq!(cfg.relationship_store, "relationships.db");
    }
}