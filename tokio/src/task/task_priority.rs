
#[derive(Copy, Clone, Debug)]
pub enum TaskPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
}

impl Default for TaskPriority{
    fn default() -> Self {
        TaskPriority::Normal
    }
}

impl TaskPriority{
    pub const VALUES: [Self; 4] = [Self::Critical, Self::High, Self::Normal, Self::Low];
}

impl From<usize> for TaskPriority{
    fn from(value: usize) -> Self {
        match value {
            0 => TaskPriority::Critical,
            1 => TaskPriority::High,
            2 => TaskPriority::Normal,
            3 => TaskPriority::Low,
            _ => TaskPriority::Low,
        }
    }
}
