use std::ops::Add;

fn period_to_duration(period: f32) -> tokio::time::Duration {
    let duration_nanos = (period * 1e9).ceil() as u64;
    return tokio::time::Duration::from_nanos(duration_nanos);
}

pub struct SystemClock {
    pub period: tokio::time::Duration,
    last_update: tokio::time::Instant,
    pub cycle_count: u128,
    // todo set this as two variables, overtime duration and undertime duration
    total_delay: i128,
}

impl SystemClock {
    pub fn from_frequency(frequency: f32) -> SystemClock {
        return SystemClock {
            period: period_to_duration(1. / frequency),
            last_update: tokio::time::Instant::now(),
            cycle_count: 0,
            total_delay: 0,
        };
    }

    pub async fn next(&mut self) {
        let elapsed = tokio::time::Instant::now().duration_since(self.last_update);
        let remaining = self
            .period
            .checked_sub(elapsed)
            .unwrap_or(tokio::time::Duration::ZERO);
        let overtime = elapsed
            .checked_sub(self.period)
            .unwrap_or(tokio::time::Duration::ZERO);

        // windows sleep granularity is 16ms, so sleep only if cumulative negative delay plus the instant negative delay is more than 16ms, otherwise hst add the negative delay
        if (-1 * self.total_delay / 1000) + (remaining.as_micros() as i128) > 32_000 {
            let dur = tokio::time::Duration::from_nanos(
                remaining.as_nanos() as u64 + (-1 * self.total_delay) as u64,
            );
            tokio::time::delay_for(dur).await;
            self.total_delay = 0;
        } else {
            self.total_delay -= remaining.as_nanos() as i128;
        }
        // elapsed > period
        if self.cycle_count > 0 {
            self.total_delay += overtime.as_nanos() as i128;
        }

        self.last_update = tokio::time::Instant::now();
        self.cycle_count += 1;
    }

    pub fn avg_delay(&self) -> i128 {
        if self.cycle_count > 0 {
            return self.total_delay / ((self.cycle_count - 1) as i128);
        }
        return 0;
    }
}
