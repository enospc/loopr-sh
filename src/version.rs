pub const VERSION: &str = match option_env!("LOOPR_VERSION") {
    Some(value) => value,
    None => "dev",
};

pub const COMMIT: &str = match option_env!("LOOPR_COMMIT") {
    Some(value) => value,
    None => "",
};

pub const DATE: &str = match option_env!("LOOPR_DATE") {
    Some(value) => value,
    None => "",
};
