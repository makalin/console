pub mod plugin;
pub mod telemetry;
pub mod storage;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Calculate average of a slice of numbers
pub fn calculate_average(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

/// Calculate standard deviation
pub fn calculate_std_deviation(numbers: &[f64]) -> f64 {
    if numbers.len() < 2 {
        return 0.0;
    }
    
    let mean = calculate_average(numbers);
    let variance = numbers.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / (numbers.len() - 1) as f64;
    
    variance.sqrt()
}

/// Convert MPH to KMH
pub fn mph_to_kmh(mph: f64) -> f64 {
    mph * 1.60934
}

/// Convert KMH to MPH
pub fn kmh_to_mph(kmh: f64) -> f64 {
    kmh / 1.60934
}

/// Convert RPM to frequency (Hz)
pub fn rpm_to_hz(rpm: f64) -> f64 {
    rpm / 60.0
}

/// Convert frequency (Hz) to RPM
pub fn hz_to_rpm(hz: f64) -> f64 {
    hz * 60.0
}

/// Calculate engine power approximation (very rough estimate)
pub fn estimate_engine_power(rpm: f64, throttle_position: f64) -> f64 {
    // This is a very simplified calculation
    // Real engine power calculation would be much more complex
    let rpm_factor = (rpm / 8000.0).clamp(0.0, 1.0);
    let throttle_factor = throttle_position / 100.0;
    rpm_factor * throttle_factor * 200.0 // Assuming 200 HP max
}

/// Calculate fuel consumption rate (L/h approximation)
pub fn estimate_fuel_consumption(rpm: f64, throttle_position: f64, engine_temp: f64) -> f64 {
    // Simplified fuel consumption calculation
    let base_consumption = rpm * 0.0001; // Base consumption per RPM
    let throttle_multiplier = 1.0 + (throttle_position / 100.0) * 2.0; // Higher throttle = more fuel
    let temp_factor = if engine_temp < 160.0 { 1.5 } else { 1.0 }; // Cold engine uses more fuel
    
    base_consumption * throttle_multiplier * temp_factor
}

/// Validate vehicle speed for reasonable range
pub fn is_valid_speed(speed: f64) -> bool {
    speed >= 0.0 && speed <= 200.0
}

/// Validate engine RPM for reasonable range
pub fn is_valid_rpm(rpm: f64) -> bool {
    rpm >= 0.0 && rpm <= 10000.0
}

/// Validate engine temperature for reasonable range
pub fn is_valid_engine_temp(temp: f64) -> bool {
    temp >= 0.0 && temp <= 300.0
}

/// Calculate distance traveled given speed and time
pub fn calculate_distance(speed_mph: f64, time_hours: f64) -> f64 {
    speed_mph * time_hours
}

/// Calculate time to destination given distance and speed
pub fn calculate_travel_time(distance_miles: f64, speed_mph: f64) -> f64 {
    if speed_mph <= 0.0 {
        return f64::INFINITY;
    }
    distance_miles / speed_mph
}

/// Format time in hours to human readable format
pub fn format_time_hours(hours: f64) -> String {
    let total_minutes = (hours * 60.0) as i32;
    let h = total_minutes / 60;
    let m = total_minutes % 60;
    
    if h > 0 {
        format!("{}h {}m", h, m)
    } else {
        format!("{}m", m)
    }
}

/// Format speed with appropriate units
pub fn format_speed(speed_mph: f64, use_metric: bool) -> String {
    if use_metric {
        let kmh = mph_to_kmh(speed_mph);
        format!("{:.1} km/h", kmh)
    } else {
        format!("{:.1} mph", speed_mph)
    }
}

/// Format RPM with appropriate units
pub fn format_rpm(rpm: f64) -> String {
    format!("{:.0} RPM", rpm)
}

/// Format temperature with appropriate units
pub fn format_temperature(temp_f: f64, use_celsius: bool) -> String {
    if use_celsius {
        let celsius = (temp_f - 32.0) * 5.0 / 9.0;
        format!("{:.1}°C", celsius)
    } else {
        format!("{:.1}°F", temp_f)
    }
}

/// Calculate gear ratio based on speed and RPM
pub fn calculate_gear_ratio(speed_mph: f64, rpm: f64) -> f64 {
    if rpm <= 0.0 {
        return 0.0;
    }
    // Simplified calculation - real gear ratios would be more complex
    speed_mph / rpm * 100.0
}

/// Estimate current gear based on speed and RPM
pub fn estimate_gear(speed_mph: f64, rpm: f64) -> i32 {
    if rpm <= 0.0 || speed_mph <= 0.0 {
        return 0;
    }
    
    let ratio = speed_mph / rpm;
    
    // Very rough gear estimation based on speed/RPM ratio
    match ratio {
        r if r < 0.01 => 1,
        r if r < 0.02 => 2,
        r if r < 0.03 => 3,
        r if r < 0.04 => 4,
        r if r < 0.05 => 5,
        _ => 6,
    }
}

/// Calculate acceleration from speed change over time
pub fn calculate_acceleration(initial_speed: f64, final_speed: f64, time_seconds: f64) -> f64 {
    if time_seconds <= 0.0 {
        return 0.0;
    }
    (final_speed - initial_speed) / time_seconds
}

/// Calculate deceleration (braking) distance
pub fn calculate_braking_distance(speed_mph: f64, deceleration_rate: f64) -> f64 {
    let speed_ms = speed_mph * 0.44704; // Convert to m/s
    (speed_ms * speed_ms) / (2.0 * deceleration_rate)
}

/// Calculate stopping time
pub fn calculate_stopping_time(speed_mph: f64, deceleration_rate: f64) -> f64 {
    let speed_ms = speed_mph * 0.44704; // Convert to m/s
    speed_ms / deceleration_rate
}

/// Check if vehicle is in motion
pub fn is_vehicle_moving(speed_mph: f64) -> bool {
    speed_mph > 1.0
}

/// Check if engine is running
pub fn is_engine_running(rpm: f64) -> bool {
    rpm > 100.0
}

/// Calculate engine load percentage
pub fn calculate_engine_load(rpm: f64, throttle_position: f64, max_rpm: f64) -> f64 {
    let rpm_load = (rpm / max_rpm) * 50.0; // RPM contributes 50% to load
    let throttle_load = throttle_position * 0.5; // Throttle contributes 50% to load
    (rpm_load + throttle_load).clamp(0.0, 100.0)
}

/// Format pressure with appropriate units
pub fn format_pressure(psi: f64, use_bar: bool) -> String {
    if use_bar {
        let bar = psi * 0.0689476;
        format!("{:.1} bar", bar)
    } else {
        format!("{:.1} PSI", psi)
    }
}

/// Format voltage
pub fn format_voltage(volts: f64) -> String {
    format!("{:.1}V", volts)
}

/// Format percentage
pub fn format_percentage(value: f64) -> String {
    format!("{:.1}%", value)
}

/// Calculate efficiency percentage
pub fn calculate_efficiency(actual: f64, theoretical: f64) -> f64 {
    if theoretical <= 0.0 {
        return 0.0;
    }
    (actual / theoretical * 100.0).clamp(0.0, 100.0)
}

/// Round to specified decimal places
pub fn round_to_decimal(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10.0_f64.powi(decimal_places as i32);
    (value * multiplier).round() / multiplier
}

/// Clamp value between min and max
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation between two values
pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t.clamp(0.0, 1.0)
}

/// Smooth step interpolation
pub fn smooth_step(start: f64, end: f64, t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    let t = t * t * (3.0 - 2.0 * t); // Smooth step function
    start + (end - start) * t
}

/// Calculate moving average
pub fn moving_average(values: &[f64], window_size: usize) -> Vec<f64> {
    if values.len() < window_size {
        return values.to_vec();
    }
    
    let mut result = Vec::new();
    for i in 0..=values.len() - window_size {
        let window = &values[i..i + window_size];
        result.push(calculate_average(window));
    }
    result
}

/// Find maximum value in slice
pub fn find_max(values: &[f64]) -> Option<f64> {
    values.iter().copied().max_by(|a, b| a.partial_cmp(b).unwrap())
}

/// Find minimum value in slice
pub fn find_min(values: &[f64]) -> Option<f64> {
    values.iter().copied().min_by(|a, b| a.partial_cmp(b).unwrap())
}

/// Calculate median of values
pub fn calculate_median(mut values: Vec<f64>) -> f64 {
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = values.len();
    if len == 0 {
        return 0.0;
    }
    if len % 2 == 0 {
        (values[len / 2 - 1] + values[len / 2]) / 2.0
    } else {
        values[len / 2]
    }
}

/// Calculate mode (most frequent value)
pub fn calculate_mode(values: &[f64]) -> Option<f64> {
    use std::collections::HashMap;
    
    let mut frequency = HashMap::new();
    for &value in values {
        *frequency.entry((value * 100.0).round() as i64).or_insert(0) += 1;
    }
    
    frequency.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&value, _)| value as f64 / 100.0)
}

/// Validate that all values are finite
pub fn validate_finite_values(values: &[f64]) -> bool {
    values.iter().all(|&x| x.is_finite())
}

/// Remove outliers using IQR method
pub fn remove_outliers(values: &[f64]) -> Vec<f64> {
    if values.len() < 4 {
        return values.to_vec();
    }
    
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let q1_index = sorted.len() / 4;
    let q3_index = 3 * sorted.len() / 4;
    let q1 = sorted[q1_index];
    let q3 = sorted[q3_index];
    let iqr = q3 - q1;
    
    let lower_bound = q1 - 1.5 * iqr;
    let upper_bound = q3 + 1.5 * iqr;
    
    values.iter()
        .filter(|&&x| x >= lower_bound && x <= upper_bound)
        .copied()
        .collect()
} 