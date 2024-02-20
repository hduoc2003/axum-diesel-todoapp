// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status_enum"))]
    pub struct StatusEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StatusEnum;

    activities (id) {
        id -> Int8,
        user_id -> Int8,
        task_id -> Int8,
        #[max_length = 512]
        title -> Varchar,
        #[max_length = 2048]
        description -> Nullable<Varchar>,
        status -> StatusEnum,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        planned_start_date -> Nullable<Timestamp>,
        planned_end_date -> Nullable<Timestamp>,
        actual_start_date -> Nullable<Timestamp>,
        actual_end_date -> Nullable<Timestamp>,
        content -> Nullable<Text>,
    }
}

diesel::table! {
    comments (id) {
        id -> Int8,
        task_id -> Int8,
        activity_id -> Nullable<Int8>,
        #[max_length = 100]
        title -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        content -> Nullable<Text>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int8,
        #[max_length = 75]
        title -> Varchar,
        #[max_length = 100]
        slug -> Varchar,
    }
}

diesel::table! {
    task_tags (task_id, tag_id) {
        task_id -> Int8,
        tag_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StatusEnum;

    tasks (id) {
        id -> Int8,
        user_id -> Int8,
        created_by -> Int8,
        updated_by -> Int8,
        #[max_length = 512]
        title -> Varchar,
        #[max_length = 2048]
        description -> Nullable<Varchar>,
        status -> StatusEnum,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        planned_start_date -> Nullable<Timestamp>,
        planned_end_date -> Nullable<Timestamp>,
        actual_start_date -> Nullable<Timestamp>,
        actual_end_date -> Nullable<Timestamp>,
        content -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        is_admin -> Bool,
        #[max_length = 50]
        first_name -> Nullable<Varchar>,
        #[max_length = 50]
        last_name -> Nullable<Varchar>,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 15]
        mobile -> Nullable<Varchar>,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        #[max_length = 150]
        password_hash -> Varchar,
        registered_at -> Timestamptz,
        last_login -> Nullable<Timestamp>,
        intro -> Nullable<Text>,
    }
}

diesel::joinable!(activities -> tasks (task_id));
diesel::joinable!(activities -> users (user_id));
diesel::joinable!(comments -> activities (activity_id));
diesel::joinable!(comments -> tasks (task_id));
diesel::joinable!(task_tags -> tags (tag_id));
diesel::joinable!(task_tags -> tasks (task_id));
diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    activities,
    comments,
    tags,
    task_tags,
    tasks,
    users,
);
