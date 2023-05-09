use std::fs;
use std::env;

use std::collections::BTreeMap;
use serde::*;

pub enum FileError<'a>
{
    FileDoesNotExist(&'a str),
    YAMLError(serde_yaml::Error)
}

pub fn write_to_file<'a, T>(data: &'a T, path: &'a str) -> Result<(), FileError<'a>>
    where T : Serialize
{
    let yaml = match serde_yaml::to_string(data) 
    {
        Ok(ok) => ok,
        Err(error) => return Err(FileError::YAMLError(error))
    };

    println!("Serialized:\n {}", yaml);

    Ok(())
}
