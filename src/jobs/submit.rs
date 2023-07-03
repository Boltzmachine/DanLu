use super::job_queue::JobQueue;

fn submit(queue: &JobQueue, command: &str) {
    queue.push_back(command);
}


pub fn batch_submit(queue: &JobQueue, commands: Vec<&str>) {
    for command in commands {
        submit(queue, command);
    }
}