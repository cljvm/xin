#[macro_use] extern crate diesel;

table! {
    #[sql_name="lx_users"]
    users (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        nick_name -> Nullable<Varchar>,
        birthday -> Nullable<Date>
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_user_auths"]
    user_auths (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        identity_type -> Varchar,
        identifier -> Varchar,
        credential -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_class"]
    class (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        nick_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_user_class"]
    user_class (user_id, class_id) {
        user_id -> Unsigned<Integer>,
        class_id -> Unsigned<Integer>,
    }
}

table! {
    #[sql_name="lx_activities"]
    activities (id) {
        id -> Unsigned<Integer>,
        title -> Varchar,
        active -> Bool,
        form -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_user_activities"]
    user_activities (user_id, activities_id) {
        user_id -> Unsigned<Integer>,
        activities_id -> Unsigned<Integer>,
        data -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_messages"]
    messages (id) {
        id -> Unsigned<Integer>,
        title -> Varchar,
        content -> Varchar,
        msg_type -> Varchar,
    }
}