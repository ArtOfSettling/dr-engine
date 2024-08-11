use crate::application::project::settings::Settings;

mod settings;

pub struct Project {
    pub(crate) project_settings: Settings,
}
