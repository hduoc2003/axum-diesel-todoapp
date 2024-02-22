
use diesel::{backend::Backend, deserialize::{self, FromSql}, pg::Pg, sql_types::Text};
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(AsRefStr)]
pub enum DatabaseType {
    #[strum(serialize = "postgres")]
    Postgres,
    #[strum(serialize = "in-memory")]
    InMemory
}

#[derive(Debug)]
#[derive(diesel_derive_enum::DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::StatusEnum"]
pub enum ProgressStatus {
    #[db_rename = "Pending"]
    Pending,
    #[db_rename = "InProgress"]
    InProgress,
    #[db_rename = "Completed"]
    Completed
}

// impl FromSql<Text, Pg> for ProgressStatus {
//     fn from_sql(bytes: <Pg as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
//         match String::from_sql(bytes)?.as_str() {
//             "Pending" => Ok(ProgressStatus::Pending),
//             "InProgress" => Ok(ProgressStatus::InProgress),
//             "Completed" => Ok(ProgressStatus::Completed),
//             x => Err(format!("Unrecognized variant {}", x).into()),
//         }
//     }
// }

// impl ToSql<Text, Pg> for ProgressStatus {
//     fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
//         match self {
//             ProgressStatus::Pending => <i32 as ToSql<Integer, Pg>>::to_sql(&0, out),
//             OrderStatus::Finished => <i32 as ToSql<Integer, Pg>>::to_sql(&1, out),
//         }
//     }
// }
