use crate::runtime_types::TaskId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledTask {
    pub id: TaskId,
    pub description: String,
    pub schedule_expr: String,
}

pub trait Scheduler: Send + Sync {
    async fn add(&self, task: ScheduledTask) -> Result<(), crate::PolicyError>;
}
