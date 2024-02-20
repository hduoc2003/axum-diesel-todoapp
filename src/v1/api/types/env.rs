use strum_macros::AsRefStr;

#[derive(AsRefStr)]
pub enum ENV {
    HOST,
    PORT,
    DATABASE,
    DATABASE_URL
}
