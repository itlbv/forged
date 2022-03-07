const LOG_LEVEL: u8 = WARN;

const DEBUG: u8 = 0;
const INFO: u8 = 1;
const WARN: u8 = 2;
const ERROR: u8 = 3;

pub fn debug(message: &str, entity_id: usize) {
    if LOG_LEVEL > 0 { return; }
    println!("INFO - [{}] - {}", entity_id, message)
}

pub fn info(message: &str, entity_id: usize) {
    if LOG_LEVEL > 1 { return; }
    println!("INFO - [{}] - {}", entity_id, message)
}

pub fn warn(message: &str, entity_id: usize) {
    if LOG_LEVEL > 2 { return; }
    println!("WARN - [{}] - {}", entity_id, message)
}

pub fn error(message: &str, entity_id: usize) {
    println!("ERROR - [{}] - {}", entity_id, message)
}