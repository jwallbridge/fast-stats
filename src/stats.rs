pub fn mean(data: &[f64]) -> Option<f64> {
    let sum: f64 = data.iter().sum();
    let count = data.len() as f64;
    match count {
        count if count > 0.0 => Some(sum/count),
        _ => None
    }
}

pub fn stddev(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - *value;
                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }
}

pub fn min(data: &[f64]) -> Option<f64> {
    if !data.is_empty() {
        Some(data.iter().copied().fold(f64::INFINITY, f64::min)) 
    } else {
        None
    }
}

pub fn max(data: &[f64]) -> Option<f64> {
    if !data.is_empty() {
        Some(data.iter().copied().fold(f64::NEG_INFINITY, f64::max))
    } else {
        None
    }
}