fn get_task_recursive(task_id: usize, tasks: &mut Vec<TaskItem>) -> (Result<(& mut TaskItem, isize), bool>) {
    for i in 0..tasks.len() {
        let task = tasks[i].id;
        if task == task_id {
            let result = (&mut tasks[i], i as isize);
            return Ok(result);
        }
    }

    for i in 0..tasks.len() {
        let result = &GroupItem::get_task_recursive(task_id, &mut tasks[i].tasks);
        if result.err().unwrap() == true {
            continue;
        }
        return Ok(result.unwrap());
    }

    return Err(true);
}