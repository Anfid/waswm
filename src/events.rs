#[derive(Debug)]
pub enum WMEvent {
    TODO,
}

#[derive(Debug)]
pub enum RunnerEvent {
    MoveWindow { id: i32, x: i32, y: i32 },
}