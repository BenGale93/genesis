use ndarray::Array;

use crate::Weight;

#[must_use]
pub fn interpolate_color(weight: Weight, colors: &[(u8, u8, u8)]) -> (u8, u8, u8) {
    let t = (weight.as_float() + 1.0) / 2.0;

    let spline_coords = Array::linspace(0.0, 1.0, colors.len());

    let mut end_color_index = 1;
    let mut sub_t = 0.0;
    for (i, val) in spline_coords.iter().enumerate() {
        if t < *val {
            let start_color_index = i - 1;
            let start_val = &spline_coords[start_color_index];
            end_color_index = i;
            sub_t = (t - start_val) / (val - start_val);
            break;
        }
    }
    if (t - 1.0).abs() < f32::EPSILON {
        sub_t = t;
        end_color_index = colors.len() - 1;
    }

    let start_color = colors[end_color_index - 1];
    let end_color = colors[end_color_index];

    let r = (f32::from(end_color.0) - f32::from(start_color.0))
        .mul_add(sub_t, f32::from(start_color.0)) as u8;
    let g = (f32::from(end_color.1) - f32::from(start_color.1))
        .mul_add(sub_t, f32::from(start_color.1)) as u8;
    let b = (f32::from(end_color.2) - f32::from(start_color.2))
        .mul_add(sub_t, f32::from(start_color.2)) as u8;

    (r, g, b)
}

pub fn rgb_to_hex(r: f32, g: f32, b: f32) -> String {
    let r_int = (r * 255.0) as u8;
    let g_int = (g * 255.0) as u8;
    let b_int = (b * 255.0) as u8;

    format!("#{r_int:02X}{g_int:02X}{b_int:02X}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weight;

    #[test]
    fn test_weight_is_one() {
        let w = weight::Weight::new(1.0).unwrap();

        let color = interpolate_color(w, &[(0, 0, 0), (100, 100, 100), (200, 200, 200)]);

        assert_eq!(color, (200, 200, 200));
    }

    #[test]
    fn rgb_to_hex_value() {
        assert_eq!("#FFD700", rgb_to_hex(1.0, 215.0 / 255.0, 0.0));
    }
}
