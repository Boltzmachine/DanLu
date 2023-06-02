fn submit(command: &str) {
    println!("Submitting command: {}", command);
    
}


pub fn batch_submit(commands: Vec<String>) {
    for command in commands {
        submit(command);
    }
}