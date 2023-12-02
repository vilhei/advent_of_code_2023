use crate::utils::{read_task_input_file, Task, TaskError};

pub struct Day23;

impl Task for Day23 {
    fn task_part_one(&self, input_file: &str) -> Result<String, TaskError> {
        // To suppress warning about unused file_content in generated files
        #[allow(unused_variables)] 
        let file_content = read_task_input_file(input_file)?;
        Err(TaskError::NotImplemented(1))
    }

    fn task_part_two(&self, input_file: &str) -> Result<String, TaskError> {
        // To suppress warning about unused file_content in generated files
        #[allow(unused_variables)] 
        let file_content = read_task_input_file(input_file)?;
        Err(TaskError::NotImplemented(2))
    }
}
