use std::{fs::File, io::{Read, Write}, ops::{Deref, DerefMut}};

use serde::{de::DeserializeOwned, Serialize};

pub struct Storage<T: Serialize+DeserializeOwned+Clone> {
    file: String,
    obj: T,
    deleted: bool
}

impl<T> Storage<T>
    where T: Serialize+DeserializeOwned+Clone {
    
    pub fn new(file: String, default: T) -> Option<Self> {
        let obj = match Self::read(file.as_str()) {
            Some(obj) => obj,
            None => {
                File::create(file.as_str()).ok()?;
                default
            },
        };

        Some(Self {
            file,
            obj,
            deleted: false
        })
    }

    pub fn write(&self) -> Option<()> {
        let mut file = File::create(&self.file).ok()?;

        let str = serde_json::to_string(&self.obj).ok()?;

        file.write_all(str.as_bytes()).ok()?;

        Some(())
    }
    fn read(path: &str) -> Option<T> {
        let mut file = File::open(path).ok()?;
        
        let mut str = String::new();
        file.read_to_string(&mut str).ok()?;
        let obj: T = serde_json::from_str(&str.as_str()).ok()?;
        Some(obj)
    }
    pub fn delete(mut self) -> Option<()> {
        self.deleted = true;
        std::fs::remove_file(self.file.as_str()).ok()
    }
}
impl<T> Deref for Storage<T>
    where T: Serialize+DeserializeOwned+Clone {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.obj
    }
}
impl<T> DerefMut for Storage<T>
    where T: Serialize+DeserializeOwned+Clone {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.write().is_none() {
            println!("Error: fail to write storage");
        }
        &mut self.obj
    }
}
impl<T> Drop for Storage<T>
    where T: Serialize+DeserializeOwned+Clone {
    fn drop(&mut self) {
        if !self.deleted {
            if self.write().is_none() {
                println!("Error: fail to write storage");
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use rand::{distributions::Alphanumeric, Rng};

    use super::Storage;

    fn get_name() -> String {
        let random = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();
        "test".to_string() + random.as_str() + ".json"
    }

    #[test]
    fn storage_empty() {
        let storage = Storage::new(get_name(), 1).unwrap();
        storage.delete();
    }

    #[test]
    fn storage_without_writing(){
        let mut storage = Storage::new(get_name(), 1).unwrap();
        *storage = 10;
        assert_eq!(10, *storage);
        storage.delete();
    }

    #[test]
    fn storage_default(){
        let storage = Storage::new(get_name(), 1).unwrap();
        assert_eq!(1, *storage);
        storage.delete();
    }

    #[test]
    fn storage_file_error(){
        assert!(Storage::new("fjdsiofjisdf/sdfjsduiofjsd/sdfjsduiofjsd".to_string(), 1).is_none());
    }

    #[test]
    fn storage_persistent() {
        let filename = get_name();
        {
            let mut storage = Storage::new(filename.clone(), 1).unwrap();
            *storage = 18;
        }
        {
            let storage = Storage::new(filename, 1).unwrap();
            assert_eq!(18, *storage);
            storage.delete();
        }
    }

    #[test]
    fn storage_delete() {
        let filename = get_name();
        {
            let mut storage = Storage::new(filename.clone(), 1).unwrap();
            *storage = 18;
            storage.delete();
        }
        {
            let storage = Storage::new(filename, 1).unwrap();
            assert_eq!(1, *storage);
            storage.delete();
        }
    }
}