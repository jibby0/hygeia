use lazy_static::lazy_static;

macro_rules! executable_name_from_env {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

/// Name of the executable, reused across project.
pub const EXECUTABLE_NAME: &str = executable_name_from_env!();

/// Default hidden configuration directory.
pub const DEFAULT_DOT_DIR: &str = concat!(".", executable_name_from_env!());

/// Return the environment variable used to find the project's config home.
pub fn home_env_variable() -> &'static str {
    lazy_static! {
        static ref HOME_ENV_VARIABLE: String =
            format!("{}_HOME", executable_name_from_env!().to_uppercase());
    }
    &HOME_ENV_VARIABLE
}

pub const INFO_FILE: &str = concat!("installed_by_", executable_name_from_env!(), ".txt");

pub const EXTRA_PACKAGES_FILENAME: &str = "extra-packages-to-install.txt";

pub const EXTRA_PACKAGES_FILENAME_CONTENT: &str = include_str!("../extra-packages-to-install.txt");
