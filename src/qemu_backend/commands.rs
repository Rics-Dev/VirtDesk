use std::process::{Command, Child};
use std::path::{Path, PathBuf};

#[derive(Debug,Copy,Clone)]
pub enum Format {
    Qcow2,
    Raw,
}

impl Format {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Qcow2 => "qcow2",
            Self::Raw => "raw",
        }
    }
}

pub struct QemuCmd {
    inner: Command,
}

impl QemuCmd {
    pub fn new() -> Self {
        Self {
            inner: Command::new("qemu-system-x86_64"),
        }
    }

    pub fn enable_kvm(mut self) -> Self {
        self.inner.arg("-enable-kvm");
        self // here we return self to allow method chaining
    }

    pub fn memory(mut self, megabytes: u32) -> Self {
        self.inner.args(["-m", &megabytes.to_string()]);
        self
    }

    pub fn smp(mut self, cores: u32) -> Self {
        self.inner.args(["-smp", &cores.to_string()]);
        self
    }


    // pub fn boot

    pub fn cdrom<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.inner.arg("-cdrom").arg(path.as_ref());
        self
    }

    pub fn drive<P: AsRef<Path>>(mut self, path: P, format: Format) -> Self {
        let drive_str = format!(
            "file={},format={}", 
            path.as_ref().display(), 
            format.as_str()
        );
        self.inner.args(["-drive", &drive_str]);
        self
    }

    pub fn display(mut self, setting: &str) -> Self {
        self.inner.args(["-display", setting]);
        self
    }

    /// Consume the builder and spawn the QEMU process in the background
    pub fn spawn(mut self) -> std::io::Result<Child> {
        println!("Spawning QEMU with generated arguments...");
        self.inner.spawn()
    }
}

// fn main() -> std::io::Result<()> {
//     // Look how incredibly clean and error-proof your main execution becomes:
//     let mut vm_process = QemuCmd::new()
//         .enable_kvm()
//         .memory(2048)
//         .smp(2)
//         .cdrom("ubuntu-24.04.1-live-server-amd64.iso")
//         .drive("disk.qcow2", Format::Qcow2)
//         .display("default")
//         .spawn()?;

//     println!("VM running under PID: {}", vm_process.id());
    
//     // Wait for the window to close
//     let status = vm_process.wait()?;
//     println!("VM stopped with exit status: {}", status);

//     Ok(())
// }