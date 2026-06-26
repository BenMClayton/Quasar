pub struct Survival {
    pub health: f32,
    pub hunger: f32,
    pub day_time: f32,
}

impl Survival {
    pub fn new() -> Self {
        Self {
            health: 100.0,
            hunger: 92.0,
            day_time: 0.18,
        }
    }

    pub fn update(&mut self, dt: f32, consumes_hunger: bool) {
        self.day_time = (self.day_time + dt / 180.0) % 1.0;

        if consumes_hunger {
            self.hunger = (self.hunger - dt * 0.45).max(0.0);
            if self.hunger <= 0.0 {
                self.health = (self.health - dt * 3.0).max(1.0);
            }
        }
    }

    pub fn feed(&mut self, amount: f32) {
        self.hunger = (self.hunger + amount).min(100.0);
    }
}
