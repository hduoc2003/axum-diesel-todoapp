use strum_macros::AsRefStr;

#[allow(non_camel_case_types)]
#[derive(AsRefStr)]
pub enum ENV {
    HOST,
    PORT,
    DATABASE,
    DATABASE_URL,
    JWT_KEY,
    JWT_EXPIRED_TIME
}
