pub (in crate::core::command_line_args) fn extract_command_line_argument_u32(
    argument_name: &str,
    args: &Vec<String>
) -> Option<u32> {
    let pattern = format!("--{argument_name}");
    let mut iter = args.iter();
    for num in iter.by_ref() {
        if num.starts_with(&pattern) {
            break;
        }
    }

    iter.next()
        .map(|arg| arg.parse::<u32>().ok())
        .flatten()
}

pub (in crate::core::command_line_args) fn extract_command_line_argument_string<'a>(
    argument_name: &str,
    args: &Vec<String>
) -> Option<String> {
    let pattern = format!("--{argument_name}");

    let mut iter = args.iter();
    for num in iter.by_ref() {
        if num.starts_with(&pattern) {
            break;
        }
    }

    iter.next().map(|a| a.to_string())
}

#[cfg(test)]
mod tests {
    use crate::core::command_line_args::command_line_parser::{extract_command_line_argument_string, extract_command_line_argument_u32};

    #[test]
    fn when_valid_can_extract_and_return_u32_value() {
        let args = vec!["--width".to_string(), "100".to_string()];
        let result = extract_command_line_argument_u32("width", &args);
        assert_eq!(result.unwrap(), 100);
    }

    #[test]
    fn when_invalid_string_cannot_extract_u32_value_returns_none() {
        let args = vec!["--width".to_string(), "1y0".to_string()];
        let result = extract_command_line_argument_u32("width", &args);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn when_negative_value_cannot_extract_u32_value_returns_none() {
        let args = vec!["--width".to_string(), "-10".to_string()];
        let result = extract_command_line_argument_u32("width", &args);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn when_valid_can_extract_and_return_string_value() {
        let args = vec!["--title".to_string(), "test title".to_string()];
        let result = extract_command_line_argument_string("title", &args);
        assert_eq!(result.unwrap(), "test title".to_string());
    }

    #[test]
    fn when_unspecified_value_cannot_extract_and_returns_none() {
        let args = vec!["--title".to_string()];
        let result = extract_command_line_argument_string("title", &args);
        assert_eq!(result.is_none(), true);
    }
}
