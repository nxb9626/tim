use sqlx::{sqlite::SqlitePool};

pub struct Storage {
    _conn: SqlitePool,
}

pub enum StorageErrors {
    NoFile,
}

impl<'a> Storage {
    pub fn start_action(&self) {}
    pub fn stop_action(&self) {}

    pub fn select(&self, _q: &str) {}

    // pub fn new(path: String) -> Result<Storage, StorageErrors> {
    // let conn = match open(path) {
    //     Ok(ayy) => ayy,
    //     Err(e) => {
    //         println!("{:?}", e);
    //         return Err(StorageErrors::NoFile);
    //     }
    // };
    // Ok(Storage { conn })
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
