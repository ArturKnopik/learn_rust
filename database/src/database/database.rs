use crate::database::database_task::DatabaseTask;
use crate::logger::logger::{LogLevel, Logger};
use std::ptr;

pub struct Database {
    tasks: Vec<Box<dyn DatabaseTask>>,
    logger: Logger,
}

impl Database {
    pub fn new() -> Self {
        let logger = Logger::new(LogLevel::Info);
        Database {
            tasks: Vec::new(),
            logger,
        }
    }

    pub fn clear_tasks(&mut self) {
        self.logger.info(format!(
            "{} - removed {} items",
            "Database: clear_tasks",
            self.tasks.len(),
        ));
        self.tasks.clear();
    }

    pub fn add_task(&mut self, task: Box<dyn DatabaseTask>) {
        self.tasks.push(task);
        self.logger.info(format!(
            "{} - addet new task, current len is {} items",
            "Database: add_task",
            self.tasks.len()
        ));
    }

    pub fn execute_all(&mut self) {
        self.logger.info(format!(
            "{} task cout {}",
            "Database: execute_all",
            self.tasks.len()
        ));
        for task in &mut self.tasks {
            self.logger.info(format!(
                "{} task addr {:?}",
                "Database: execute_all",
                ptr::addr_of!(task)
            ));
            task.execute();
        }
        self.clear_tasks();
    }

    pub fn get_last_task(&self) -> Option<&Box<dyn DatabaseTask>> {
        self.tasks.last()
    }

    pub fn print_size(&self) {
        println!("{} - {} items", "Database: print_size", self.tasks.len());
    }
}