use lazy_static::lazy_static;
use async_trait::async_trait;
use spin::Mutex;
use alloc::{vec::Vec, string::String, boxed::Box};
use crate::std::{self, application::{Error, Application}};


lazy_static! {
    pub static ref FILESYSTEM: Mutex<Box<File>> = Mutex::new(Box::new(File::new(
        String::from(""), // root
        FileType::Dir(Directory::new()),
    )));
}

pub type Directory = Vec<Box<File>>;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub data: FileType,
}
impl File {
    pub fn new(name: String, data: FileType) -> Self {
        Self { name, data }
    }
}

#[derive(Debug)]
pub enum FileType {
    Dir(Directory),
    Txt(String),
    Exe(Apppp),
}




pub fn mkfs() {
    let mut fs = FILESYSTEM.lock();

    match fs.data {
        FileType::Dir(ref mut dir) => {
            dir.push(Box::new(
                File::new(
                    String::from("hello there"),
                    FileType::Txt(String::from("this is a basic text file")),
                )
            ));
            dir.push(Box::new(
                File::new(
                    String::from("function that prints out an integer"),
                    FileType::Exe(Apppp::new()),
                )
            ));
        }
        _ => {
            ()
        }
    }
}


#[derive(Debug)]
pub struct Apppp {}

#[async_trait]
impl std::application::Application for Apppp {
    fn new() -> Self {
        Self {}
    }
    async fn run(&mut self, _: Vec<String>) -> Result<(), Error> {
        Ok(())
    }
}

/* 
lazy_static! {
    pub static ref FILESYSTEM: Mutex<Filesystem> = Mutex::new(Filesystem::new());
}

enum FsError {
    FileNotFound,
    AlreadyExists,
    InvalidPath,
}

pub type Path = String;

impl Path {
    pub fn new(path: &str) -> Result<Path, FsError> {
        if path.is_empty() {
            return Err(FsError::FileNotFound);
        }
        Ok(path.to_string())
    }
    pub fn dirs(&self) -> Vec<String> {
        self.split('/').map(|s| s.to_string()).collect()
    }
}


pub struct Filesystem {
    pub root: Vec<File>
}



pub type Directory = Vec<File>;

impl Directory {
    pub fn new(name: String, files: Vec<File>) -> Self {
        Self { name, files }
    }
    pub fn size(&self) -> usize {
        self.files.len()
    }
    pub fn mkdir(&mut self, name: &str, location: Path) -> Result<(), FsError> {
        if self.exists(location.as_str()) {
            return Err(FsError::AlreadyExists);
        }
        self.files.push(File::new(name, FileType::Directory(Box::new(Directory::new()))))
    }
}


pub enum FileType {
    Directory(Box<Directory>),
    TextFile(Box<TextFile>),
    BinaryFile(Box<BinaryFile>),
    Executable(fn()),
}


pub struct File {
    pub name: String,
    pub file_type: FileType,
}
impl File {
    pub fn new(name: String, file_type: FileType) -> Self {
        Self { name, file_type }
    }
}



pub struct TextFile {
    pub name: String,
    pub data: String,
}
impl TextFile {
    pub fn new(name: String, data: String) -> Self {
        Self { name, data }
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
}


*/