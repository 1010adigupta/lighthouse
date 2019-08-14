use prometheus::{HistogramOpts, HistogramTimer, Opts};

pub use prometheus::{Histogram, IntCounter, IntGauge, Result};

pub fn gather() -> Vec<prometheus::proto::MetricFamily> {
    prometheus::gather()
}

pub fn try_create_int_counter(name: &str, help: &str) -> Result<IntCounter> {
    let opts = Opts::new(name, help);
    let counter = IntCounter::with_opts(opts)?;
    prometheus::register(Box::new(counter.clone()))?;
    Ok(counter)
}

pub fn try_create_int_gauge(name: &str, help: &str) -> Result<IntGauge> {
    let opts = Opts::new(name, help);
    let gauge = IntGauge::with_opts(opts)?;
    prometheus::register(Box::new(gauge.clone()))?;
    Ok(gauge)
}

pub fn try_create_histogram(name: &str, help: &str) -> Result<Histogram> {
    let opts = HistogramOpts::new(name, help);
    let histogram = Histogram::with_opts(opts)?;
    prometheus::register(Box::new(histogram.clone()))?;
    Ok(histogram)
}

pub fn start_timer(histogram: &Result<Histogram>) -> Option<HistogramTimer> {
    if let Ok(histogram) = histogram {
        Some(histogram.start_timer())
    } else {
        None
    }
}

pub fn stop_timer(timer: Option<HistogramTimer>) {
    timer.map(|t| t.observe_duration());
}

pub fn inc_counter(counter: &Result<IntCounter>) {
    if let Ok(counter) = counter {
        counter.inc();
    }
}

pub fn inc_counter_by(counter: &Result<IntCounter>, value: i64) {
    if let Ok(counter) = counter {
        counter.inc_by(value);
    }
}

pub fn set_gauge(gauge: &Result<IntGauge>, value: i64) {
    if let Ok(gauge) = gauge {
        gauge.set(value);
    }
}

pub fn observe(histogram: &Result<Histogram>, value: f64) {
    if let Ok(histogram) = histogram {
        histogram.observe(value);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
