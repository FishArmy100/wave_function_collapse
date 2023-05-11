use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serde::*;
use serde::de::DeserializeOwned;

use crate::tile_map::{TileMap};

pub fn serialize_to_file<T>(data: &T, path: &str)
    where T : Serialize
{
    let path = Path::new(path);

    let yaml = match serde_yaml::to_string(data) 
    {
        Ok(ok) => ok,
        Err(error) => panic!("{}", error)
    };

    let mut file = match File::options().create(true).write(true).open(path)
    {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", path.display(), why)
    };

    match file.write_all(yaml.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
        Ok(_) => ()
    }
}

pub fn print_serialized<T>(data: &T) -> String
    where T : Serialize
{
    let yaml = match serde_yaml::to_string(data) 
    {
        Ok(ok) => ok,
        Err(error) => panic!("{}", error)
    };

    yaml
}

pub fn deserialize_from_file<T>(path: &str) -> T
    where T : DeserializeOwned
{
    let path = Path::new(path);

    let mut file = match File::options().read(true).open(path)
    {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", path.display(), why)
    };

    let mut yaml = String::new();
    match file.read_to_string(&mut yaml) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
        Ok(_) => ()
    }

    println!("{}", yaml);

    match serde_yaml::from_str::<T>(&yaml)
    {
        Ok(ok) => ok,
        Err(error) => panic!("{}", error)
    }
}

pub fn save_map(entity: &TileMap, path: &str)
{
    todo!()
}

pub fn load_map(path: &str) -> TileMap
{
    todo!()
}
