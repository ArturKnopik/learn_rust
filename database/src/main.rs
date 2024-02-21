mod database;
mod logger;

use database::database_task::DatabaseRead;
use database::database_task::DatabaseWrite;
use logger::logger::LogLevel;
use logger::logger::Logger;

fn main() {
    println!("--==TESTING==--");
    let mut db = database::database::Database::new();
    db.clear_tasks();
    db.add_task(Box::new(DatabaseRead::default()));
    db.add_task(Box::new(DatabaseWrite::default()));

    let task = db.get_last_task();
    task.unwrap().execute();

    db.print_size();
    db.execute_all();

    let mut logger = Logger::new(LogLevel::Info);
    logger.log(LogLevel::Warning, "Test selected level".to_string());

    logger.info("Test 1 into".to_string());
    logger.warning("Test 1 warning".to_string());
    logger.error("Test 1 error".to_string());

    logger.set_new_level(LogLevel::Warning);

    logger.info("Test 2 into".to_string());
    logger.warning("Test 2 warning".to_string());
    logger.error("Test 2 error".to_string());
}
