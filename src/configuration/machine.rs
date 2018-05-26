use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MachineConfiguration {
    pub name: String,
    pub uuid: Option<Uuid>,
    pub cores: i32,
    pub memory: u64,
}
