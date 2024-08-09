mod command_line_parser;

use tracing::info;
use command_line_parser::extract_command_line_argument_u32;
use command_line_parser::extract_command_line_argument_string;

#[derive(Debug)]
pub struct CommandLineArgs {
    pub title: Option<String>,
    pub working_directory: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub all_args: Vec<String>
}

impl CommandLineArgs {
    pub fn build(args: &Vec<String>) -> CommandLineArgs {
        let command_line_args = CommandLineArgs {
            title: extract_command_line_argument_string("title", &args),
            working_directory: extract_command_line_argument_string("working_directory", &args),
            width: extract_command_line_argument_u32("width", &args),
            height: extract_command_line_argument_u32("height", &args),
            all_args: args.to_vec()
        };

        info!("input command_line_args are: {args:?}");
        info!("parsed command_line_args are: {command_line_args:?}");

        command_line_args
    }
}

#[cfg(test)]
mod tests {
    use crate::core::CommandLineArgs;

    #[test]
    fn when_valid_title_can_extract_sets_correct_data() {
        let args = vec!["-title=some-title".to_string()];
        let command_line_args = CommandLineArgs::build(&args);
        assert_eq!(command_line_args.title.unwrap(), "some-title");
        assert!(command_line_args.working_directory.is_none());
        assert!(command_line_args.width.is_none());
        assert!(command_line_args.height.is_none());
        assert_eq!(command_line_args.all_args, args.to_vec());
    }

    #[test]
    fn when_valid_working_directory_can_extract_sets_correct_data() {
        let args = vec!["-working_directory=a working directory".to_string()];
        let command_line_args = CommandLineArgs::build(&args);
        assert!(command_line_args.title.is_none());
        assert_eq!(command_line_args.working_directory.unwrap(), "a working directory");
        assert!(command_line_args.width.is_none());
        assert!(command_line_args.height.is_none());
        assert_eq!(command_line_args.all_args, args.to_vec());
    }

    #[test]
    fn when_valid_width_can_extract_sets_correct_data() {
        let args = vec!["-width=100".to_string()];
        let command_line_args = CommandLineArgs::build(&args);
        assert!(command_line_args.title.is_none());
        assert!(command_line_args.working_directory.is_none());
        assert_eq!(command_line_args.width.unwrap(), 100);
        assert!(command_line_args.height.is_none());
        assert_eq!(command_line_args.all_args, args.to_vec());
    }

    #[test]
    fn when_valid_height_can_extract_sets_correct_data() {
        let args = vec!["-height=100".to_string()];
        let command_line_args = CommandLineArgs::build(&args);
        assert!(command_line_args.title.is_none());
        assert!(command_line_args.working_directory.is_none());
        assert!(command_line_args.width.is_none());
        assert_eq!(command_line_args.height.unwrap(), 100);
        assert_eq!(command_line_args.all_args, args.to_vec());
    }

    #[test]
    fn when_all_valid_can_extract_sets_correct_data() {
        let args = vec![
            "-title=some-title".to_string(),
            "-working_directory=work".to_string(),
            "-width=50".to_string(),
            "-height=60".to_string(),
            "-something=asd".to_string()
        ];

        let command_line_args = CommandLineArgs::build(&args);
        assert_eq!(command_line_args.title.unwrap(), "some-title");
        assert_eq!(command_line_args.working_directory.unwrap(), "work");
        assert_eq!(command_line_args.width.unwrap(), 50);
        assert_eq!(command_line_args.height.unwrap(), 60);
        assert_eq!(command_line_args.all_args, args.to_vec());
    }

    #[test]
    fn when_none_valid_can_extract_sets_correct_data() {
        let args = vec!["-unreal=100".to_string()];
        let command_line_args = CommandLineArgs::build(&args);
        assert!(command_line_args.title.is_none());
        assert!(command_line_args.working_directory.is_none());
        assert!(command_line_args.width.is_none());
        assert!(command_line_args.height.is_none());
        assert_eq!(command_line_args.all_args, args.to_vec());
    }
}
