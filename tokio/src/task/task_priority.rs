
#[derive(Copy, Clone, Debug)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Default for TaskPriority{
    fn default() -> Self {
        TaskPriority::Normal
    }
}

impl TaskPriority{
    pub const VALUES: [Self; 4] = [Self::Critical, Self::High, Self::Normal, Self::Low];
}
