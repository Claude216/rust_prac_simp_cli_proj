use std::io;

// this is the struct to store hte task content and the task id
struct Task {
    id: usize,
    content: String,
}



fn main() {
    println!("Hello, world!");
}

// add one task to the task list
fn add_task(tasks: &mut Vec<Task>, content: String) {
    let id = tasks.len() + 1; // get the current task id by increase the current max id by 1
    tasks.push(Task {id, content}); // push the new task to the task list
}

// list all the tasks in the task list (by printing on the console)
fn list_tasks(tasks: &mut Vec<Task>) {
    for task in tasks {
        println!("{}: {}", task.id, task.content);
    }
}

