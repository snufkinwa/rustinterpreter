// macros.rs
#[macro_export]
macro_rules! format_number {
    ($num_str:expr) => {{
        if let Ok(num) = $num_str.parse::<f64>() {
            if num.fract() == 0.0 {
                format!("{:.1}", num)
            } else {
                $num_str.trim_end_matches('0').trim_end_matches('.').to_string()
            }
        } else {
            $num_str.to_string()
        }
    }};
}

#[macro_export]
macro_rules! format_number_test {
    ($num:expr) => {{
        if $num.fract() == 0.0 {
            // If the number is an integer, format it with one decimal place (e.g., 99 -> 99.0)
            format!("{:.1}", $num)
        } else {
            // For non-integer numbers, format to two decimal places, and remove trailing zeros
            let formatted = format!("{:.2}", $num);
            formatted.trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }};
}




