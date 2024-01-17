pub struct BootLoader {}
impl BootLoader {
    fn load(&self) {
        println!("loaded operating system via EFI");
    }
}
pub struct InitSystem {}
impl InitSystem {
    fn init(&self) {
        println!("Systemd startup");
    }
}

pub struct SessionManager {}
impl SessionManager {
    fn login(&self) {
        println!("SDDM initiated");
    }
}

pub struct PCFacade {
    boot_loader: BootLoader,
    init_system: InitSystem,
    session_manager: SessionManager,
}

impl PCFacade {
    pub fn create() -> Self {
        Self { boot_loader: BootLoader {  }, init_system: InitSystem {  }, session_manager: SessionManager {  } }
    }

    pub fn boot(&self) {
        self.boot_loader.load();
        self.init_system.init();
        self.session_manager.login();
    }
}
