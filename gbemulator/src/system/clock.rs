use std::time::Duration;
const WIN_MIN_SLEEP_TIME: Duration = Duration::from_millis(1);

fn period_to_duration(period: f32) -> Duration {
    let duration_nanos = (period * 1e9).ceil() as u64;
    return Duration::from_nanos(duration_nanos);
}

pub struct SystemClock {
    pub period: Duration,
    last_update: std::time::Instant,
    pub cycle_count: u128,
    pub sleep_count: u128,
    total_delay: i128,
    undertime_duration: Duration,
    overtime_duration: Duration,
}

impl SystemClock {
    pub fn from_frequency(frequency: f32) -> SystemClock {
        return SystemClock {
            period: period_to_duration(1. / frequency),
            last_update: std::time::Instant::now(),
            cycle_count: 0,
            sleep_count: 0,
            total_delay: 0,
            undertime_duration: Duration::ZERO,
            overtime_duration: Duration::ZERO,
        };
    }

    fn sleep_if_needed(&mut self) -> Duration {
        let sleep_start: std::time::Instant = std::time::Instant::now();
        let sleep_duration = self
            .undertime_duration
            .checked_sub(self.overtime_duration)
            .unwrap_or(Duration::ZERO);
        if sleep_duration > WIN_MIN_SLEEP_TIME {
            std::thread::sleep(WIN_MIN_SLEEP_TIME);
        }
        let sleep_duration = std::time::Instant::now().duration_since(sleep_start);
        self.undertime_duration = self
            .undertime_duration
            .checked_sub(sleep_duration)
            .unwrap_or(Duration::ZERO);
        self.overtime_duration = Duration::ZERO;
        self.sleep_count += 1;
        sleep_duration
    }

    pub fn next(&mut self) {
        let slept_duration = self.sleep_if_needed();
        self.cycle_count += 1;
        if self.cycle_count > 1 {
            let elapsed = self.last_update.elapsed();
            self.last_update = std::time::Instant::now();

            let mut overtime = elapsed.checked_sub(self.period).unwrap_or(Duration::ZERO);
            if overtime > Duration::ZERO {
                overtime = overtime
                    .checked_sub(slept_duration)
                    .unwrap_or(Duration::ZERO);
                /*.checked_add(print_duration)
                .unwrap_or(Duration::ZERO);*/
            }

            if overtime > Duration::ZERO {
                self.overtime_duration = self
                    .overtime_duration
                    .checked_add(overtime)
                    .unwrap_or(Duration::ZERO);
                self.total_delay += overtime.as_nanos() as i128;
            } else {
                let remaining = self.period.checked_sub(elapsed).unwrap_or(Duration::ZERO);
                self.undertime_duration = self
                    .undertime_duration
                    .checked_add(remaining)
                    .unwrap_or(Duration::ZERO);
                self.total_delay -= remaining.as_nanos() as i128;
            }
        }
    }

    pub fn avg_delay(&self) -> i128 {
        return self.total_delay / self.cycle_count as i128;
    }

    pub fn sleep_count(&self) -> u128 {
        self.sleep_count
    }
}
