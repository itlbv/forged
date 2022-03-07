pub fn info(message: String, entity_id: usize) {
    println!("INFO - [{}] - {}", entity_id, message)
}

pub fn error(message: String, entity_id: usize) {
    println!("ERROR - [{}] - {}", entity_id, message)
}