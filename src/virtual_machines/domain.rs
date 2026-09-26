#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatingSystem {
    Linux,
    Windows,
    MacOs,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualMachineState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualMachine {
    pub name: String,
    // pub os: OperatingSystem,
    pub os: String,
    pub memory: u64,
    pub storage_gb: u64,
    pub vcpus: u8,
    pub state: VirtualMachineState,
}

impl VirtualMachine {
    pub fn new(
        name: impl Into<String>,
        os: impl Into<String>,
        memory: u64,
        storage_gb: u64,
    ) -> Self {
        Self {
            name: name.into(),
            os: os.into(),
            memory,
            storage_gb,
            vcpus: 1,
            state: VirtualMachineState::Stopped,
        }
    }
}
