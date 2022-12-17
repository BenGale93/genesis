pub(super) fn low_high_tuple(tuple: (f32, f32), name: &str) -> Option<String> {
    if tuple.0 >= tuple.1 {
        Some(format!(
            "Lower limit of '{name}' is higher than upper limit"
        ))
    } else {
        None
    }
}

pub(super) fn min_value<T: PartialOrd + Clone + std::fmt::Display>(
    floor: T,
    value: T,
    name: &str,
) -> Option<String> {
    if value < floor {
        Some(format!("The value '{name}' must be larger than {floor}."))
    } else {
        None
    }
}

pub(super) fn low_high<T: PartialOrd + Clone>(
    lower: T,
    higher: T,
    low_name: &str,
    high_name: &str,
) -> Option<String> {
    if higher <= lower {
        Some(format!(
            "The value '{high_name}' must be larger than '{low_name}'"
        ))
    } else {
        None
    }
}

pub(super) fn attribute_overlap(
    lower: (f32, f32, usize),
    higher: (f32, f32, usize),
    low_name: &str,
    high_name: &str,
) -> Option<String> {
    let ceil_lower = lower.0.max(lower.1);
    let floor_higher = higher.0.min(higher.1);

    if floor_higher <= ceil_lower {
        Some(format!("The highest value of '{low_name}' should be lower than the lowest value of '{high_name}'."))
    } else {
        None
    }
}

pub(super) fn attribute_limit(
    attr: (f32, f32, usize),
    validator: (Option<f32>, Option<f32>),
    name: &str,
) -> Vec<Option<String>> {
    let mut messages = vec![];
    let floor = attr.0.min(attr.1);
    let ceil = attr.0.max(attr.1);
    let (lower, upper) = validator;

    if let Some(v) = lower {
        if floor <= v {
            let message = Some(format!(
                "The lowest value of '{name}' must be higher than {v}."
            ));
            messages.push(message);
        }
    }
    if let Some(v) = upper {
        if v < ceil {
            let message = Some(format!(
                "The highest value of '{name}' must be lower than {v}."
            ));
            messages.push(message);
        }
    }
    messages
}
