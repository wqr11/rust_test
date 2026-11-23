// 5 min
// Write TODO app

use std::fs::File;
use std::io::{BufReader, Error, Read, Write};
use std::time::SystemTime;

const TASKS_CONTENT_SEPARATOR: &str = "|";

struct Task {
    title: String,
    content: String,
    created_at: SystemTime,
}

impl Task {
    fn new(title: &str, content: &str) -> Self {
        Self {
            title: String::from(title),
            content: String::from(content),
            created_at: SystemTime::now(),
        }
    }
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn add(&mut self, task: Task) -> () {
        self.tasks.push(task);
    }

    fn list(&self) -> () {
        if self.tasks.is_empty() {
            println!("NO TASKS");
            return;
        }
        for task in &self.tasks {
            println!("Task: {}\n{}", task.title, task.content);
        }
    }

    fn export(&self, filename: &str) -> Result<(), Error> {
        let mut file = File::create(filename)?;
        file.lock()?;
        for task in &self.tasks {
            writeln!(
                file,
                "{}{}{}",
                task.title, TASKS_CONTENT_SEPARATOR, task.content
            )?;
        }
        file.unlock()?;
        Ok(())
    }

    fn import(&mut self, filename: &str) -> Result<(), Error> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut s = String::new();
        reader.read_to_string(&mut s)?;
        let loaded_tasks: Vec<&str> = s.split("\n").collect();
        for task in loaded_tasks {
            if task.is_empty() {
                continue;
            }
            let t: Vec<&str> = task.split(TASKS_CONTENT_SEPARATOR).collect();
            assert_eq!(t.len(), 2);
            self.add(Task::new(t[0], t[1]));
        }

        Ok(())
    }

    fn clear(&mut self) -> () {
        self.tasks.clear();
    }

    fn new() -> Self {
        Self { tasks: vec![] }
    }
}

fn main() {
    let mut manager = TaskManager::new();

    let task = Task::new("First task", "Content yo");

    const FILE: &str = "local_file.txt";

    manager.add(task);

    manager.list();

    manager.export(FILE).unwrap();

    manager.clear();

    manager.list();

    manager.import(FILE).unwrap();

    manager.list();
}
