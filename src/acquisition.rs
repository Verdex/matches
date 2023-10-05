
use std::fs;
use std::path::PathBuf;

use structuralize::data::*;

use crate::parsing;

// TODO : check to see if there's a .gitignore and then check that to see which files should be ignored
// TODO : flag to ignore the .gitignore
// TODO : flag to go through .git directory
// TODO : flag to process only the file from the given path
// TODO : flag to process only stdin (and I guess a way to specify how to parse?)
// TODO : will need to ignore unknown file extensions

pub fn get_data_from_dir() -> std::io::Result<Data> {
    dir_data(".".into(), ".".into())
}

fn dir_data( dir : PathBuf, dir_name : Box<str> ) -> std::io::Result<Data> {
    let mut directory_data : Vec<Data> = vec![];
    for item in fs::read_dir(dir)? {
        let item = item?;

        if item.file_name() == ".git" {
            continue;
        }

        let ft = item.file_type()?;
        if ft.is_dir() {
            directory_data.push(dir_data(item.path(), item.file_name().into_string().unwrap().into())?);
        }
        else if ft.is_file() {
            let path = item.path();
            // TODO 
            let name : Box<str> = item.file_name().into_string().unwrap().into();
            if let Some(ext) = path.extension() {
                if ext == "cs" {
                    let file_input = fs::read_to_string(path)?;
                    let data = parsing::c_sharp::parse(&file_input).unwrap();
                    directory_data.push(
                        Data::Cons { name: "file".into()
                                   , params: vec![Data::String(name), data] 
                                   });
                }
                else {
                    directory_data.push(
                        Data::Cons { name: "file".into() 
                                   , params: vec![Data::String(name), Data::Symbol("unsupported".into())]
                                   });
                }
            }
        }
    }
    Ok(Data::Cons { name: "directory".into()
                  , params: vec![Data::String(dir_name), Data::List(directory_data)]
                  })
}
