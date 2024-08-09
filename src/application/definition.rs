use tracing::info;
use crate::constants;
use crate::core::CommandLineArgs;

#[derive(Debug)]
pub struct Definition {
    pub(crate) title: String,
    pub(crate) working_directory: String,
    pub(crate) width: u32,
    pub(crate) height: u32
}

impl Definition {
    pub fn default() -> Self {
        Definition {
            title: constants::application::APPLICATION_TITLE.to_string(),
            working_directory: constants::application::APPLICATION_WORKING_DIRECTORY.to_string(),
            width: constants::application::APPLICATION_WIDTH,
            height: constants::application::APPLICATION_HEIGHT
        }
    }

    pub fn with_title(self, title: String) -> Self {
        Definition { title, ..self }
    }

    pub fn with_working_directory(self, working_directory: String) -> Self {
        Definition { working_directory, ..self }
    }

    pub fn with_width(self, width: u32) -> Self {
        Definition { width, ..self }
    }

    pub fn with_height(self, height: u32) -> Self {
        Definition { height, ..self }
    }

    pub fn override_with(self, command_line_args: CommandLineArgs) -> Self {
        info!("overriding with: {command_line_args:?}");

        let CommandLineArgs { title, working_directory, width, height, .. } = command_line_args;
        let mut application_definition = Definition { ..self };

        if let Some(title) = title {
            application_definition = application_definition.with_title(title)
        }

        if let Some(working_directory) = working_directory {
            application_definition = application_definition.with_working_directory(working_directory)
        }

        if let Some(width) = width {
            application_definition = application_definition.with_width(width)
        }

        if let Some(height) = height {
            application_definition = application_definition.with_height(height);
        }

        info!("resultant definition: {application_definition:?}");

        application_definition
    }
}
