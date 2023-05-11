use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serde::*;
use serde::de::DeserializeOwned;

use crate::tile_map::TileMap;

pub fn serialize_to_file<T>(data: &T, path: &str)
    where T : Serialize
{
    let path = Path::new(path);

    let json = match serde_json::to_string(data) 
    {
        Ok(ok) => ok,
        Err(error) => panic!("{}", error)
    };

    let mut file = match File::options().create(true).write(true).open(path)
    {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", path.display(), why)
    };

    match file.write_all(json.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
        Ok(_) => ()
    }
}

pub fn print_serialized<T>(data: &T) -> String
    where T : Serialize
{
    let yaml = match serde_json::to_string(data) 
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

    let mut json = String::new();
    match file.read_to_string(&mut json) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
        Ok(_) => ()
    }

    match serde_json::from_str::<T>(&json)
    {
        Ok(ok) => ok,
        Err(error) => panic!("{}", error)
    }
}
