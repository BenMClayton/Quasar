
#[derive(Clone, Copy, Debug)]
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
            // Cap hunger depletion to prevent excessive drain from high dt values
            let max_drain_per_frame = 50.0;
            let actual_dt = dt.min(max_drain_per_frame / 0.45);
            
            self.hunger = (self.hunger - actual_dt * 0.45).max(0.0);
            
            // Apply starvation damage only if hunger is depleted, using the same capped time delta
            if self.hunger <= 0.0 {
                self.health = (self.health - actual_dt * 3.0).max(1.0);
            }
        }
    }

    pub fn feed(&mut self, amount: f32) {
        self.hunger = (self.hunger + amount).min(100.0);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn survival_new_initializes_with_expected_values() {
        // Verify that new() creates a Survival instance with the correct initial values
        let survival = Survival::new();

        assert_eq!(survival.health, 100.0);
        assert_eq!(survival.hunger, 92.0);
        assert!((survival.day_time - 0.18).abs() < f32::EPSILON * 10.0);
    }


}
