pub trait DatabaseTask {
    fn execute(&self) -> i32;
}

#[derive(Default)]
pub struct DatabaseRead;
impl DatabaseTask for DatabaseRead {
    fn execute(&self) -> i32 {
        println!("DatabaseTask: databaseRead");
        0
    }
}

#[derive(Default)]
pub struct DatabaseWrite;
impl DatabaseTask for DatabaseWrite {
    fn execute(&self) -> i32 {
        println!("DatabaseTask: DatabaseWrite");
        0
    }
}
