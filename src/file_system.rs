use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use serde::*;
use serde::de::DeserializeOwned;

pub type YamlError = serde_yaml::Error;

pub fn serialize_to_file<T>(data: &T, path: &str) -> Result<(), YamlError>
    where T : Serialize
{
    let path = Path::new(path);

    let yaml = match serde_yaml::to_string(data) 
    {
        Ok(ok) => ok,
        Err(error) => return Err(error)
    };

    let mut file = match File::create(path)
    {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", path.display(), why)
    };

    match file.write_all(yaml.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
        Ok(_) => ()
    }

    Ok(())
}

pub fn deserialize_from_file<T>(path: &str) -> Result<T, YamlError>
    where T : DeserializeOwned
{
    let path = Path::new(path);

    let mut file = match File::create(path)
    {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", path.display(), why)
    };

    let mut yaml = String::new();
    match file.read_to_string(&mut yaml) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
        Ok(_) => ()
    }

    serde_yaml::from_str::<T>(&yaml) 
}
