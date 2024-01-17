#[derive(Clone, Copy)]
pub struct WorkPiece {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub id: u32,
}

pub struct Collections {
    pub display_time: Vec<WorkPiece>,
    pub set_hours: Vec<WorkPiece>,
    pub set_minutes: Vec<WorkPiece>,
}

impl Collections {
    pub fn create() -> Self {
        Self {
            display_time: Vec::new(),
            set_hours: Vec::new(),
            set_minutes: Vec::new(),
        }
    }

    pub fn add_workpiece(&mut self, workpiece: WorkPiece) {
        self.display_time.push(workpiece);
    }

    pub fn change_mode(&mut self) {
        let mut buffer = Vec::new();
        buffer.extend(self.display_time.clone());
        self.display_time.clear();

        self.display_time.extend(self.set_minutes.clone());
        self.set_minutes.clear();

        self.set_minutes.extend(self.set_hours.clone());
        self.set_hours.clear();

        self.set_hours.extend(buffer);
    }

    pub fn tick(&mut self) {
        for piece in self.display_time.iter_mut() {
            piece.hours += 1;
            piece.minutes += 2;
            piece.seconds += 3;
            println!(
                "tickeroni {} {} {} id: {}",
                piece.hours, piece.minutes, piece.seconds, piece.id
            );
        }
    }

    pub fn increment(&mut self) {
        for item in self.set_hours.iter_mut() {
            item.hours += 1;
            println!("set hours {} id: {}", item.hours, item.id);
        }
        for item in self.set_minutes.iter_mut() {
            item.minutes += 1;
            println!("set minutes {} id: {}", item.minutes, item.id);
        }
    }
}
