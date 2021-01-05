use std::time::Duration;

#[derive(Default)]
pub struct TimeStepState {
    next_duration: Option<Duration>,
    max_duration: Option<Duration>,
}

impl TimeStepState {
    /// The amount of time the current step takes
    pub fn get_step_duration(&self) -> Duration {
        Duration::min(
            self.next_duration
                .expect("did not call limit_step_duration() after initialize_step()"),
            self.max_duration.expect("did not initialize_step()"),
        )
    }

    pub fn initialize_step(&mut self, max_duration: Duration) {
        self.max_duration = Some(max_duration);
        self.next_duration = None;
    }

    pub fn limit_step_duration(&mut self, min_duration: Duration) {
        self.next_duration = Some(match self.next_duration {
            None => min_duration,
            Some(current_min) => Duration::min(current_min, min_duration),
        });
    }

    /// This is only for debugging purposes
    pub fn get_step_duration_limit(&self) -> Option<Duration> {
        self.next_duration
    }
}
