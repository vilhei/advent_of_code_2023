use std::fs;

#[derive(Debug)]
pub enum TaskError {
    InvalidFilePath(String),
    NotImplemented(usize),
}

pub fn read_task_input_file(path: &str) -> Result<String, TaskError> {
    let Ok(file_contents) = fs::read_to_string(path) else {
        return Err(TaskError::InvalidFilePath(path.to_string()));
    };
    Ok(file_contents)
}

pub trait Task {
    fn task_part_one(&self, _input_file: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(1))
    }
    fn task_part_two(&self, _input_file: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(2))
    }
}
