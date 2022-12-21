pub fn low_high_tuple(tuple: (f32, f32), name: &str) -> Option<String> {
    if tuple.0 >= tuple.1 {
        Some(format!(
            "Lower limit of '{name}' is higher than upper limit"
        ))
    } else {
        None
    }
}

pub fn min_value<T: PartialOrd + Clone + std::fmt::Display>(
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

pub fn low_high<T: PartialOrd + Clone>(
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

pub fn between<T: PartialOrd + Clone + std::fmt::Display>(
    value: T,
    min: T,
    max: T,
    name: &str,
) -> Option<String> {
    if value < min || value > max {
        Some(format!(
            "The value '{name}' must be larger than {min} and smaller then {max}."
        ))
    } else {
        None
    }
}

pub fn attribute_limit(
    floor: f32,
    ceil: f32,
    validator: (Option<f32>, Option<f32>),
    name: &str,
) -> Vec<Option<String>> {
    let mut messages = vec![];
    if ceil <= floor {
        messages.push(Some(format!(
            "The values of '{name}' are in the wrong order."
        )));
        return messages;
    }
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
