use advents::utils::TaskError;

pub fn handle_answer(res: &Result<String, TaskError>, day: u64) {
    if let Ok(answer) = res {
        println!("Day {day} answer :\n{answer}");
        return;
    }
    match res.as_ref().unwrap_err() {
        TaskError::InvalidFilePath(reason) => panic!("Invalid file path :\n {reason}"),
        TaskError::NotImplemented(task_n) => panic!("Task {task_n} not implemented"),
    }
}
