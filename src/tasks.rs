use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Error, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at = Utc::now();
        Task { text, created_at }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

pub fn add_task(file_path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    let mut tasks = collect_tasks(&file)?;
    tasks.push(task);

    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn complete_task(file_path: PathBuf, task_position: usize) -> Result<()> {
    let file = OpenOptions::new().read(true).write(true).open(file_path)?;

    let mut tasks = collect_tasks(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid Task Id",
        ));
    }
    tasks.remove(task_position - 1);

    // truncate the file before writing over it.
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(file_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("No tasks");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }
    Ok(())
}

fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    // rewind the file before write over it.
    file.seek(SeekFrom::Start(0))?;
    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}
