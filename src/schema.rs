table! {
    #[sql_name="lx_user"]
    user (id) {
        id -> Integer,
        name -> Varchar,
        nick_name -> Nullable<Varchar>,
        birthday -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_user_auth"]
    user_auth (id) {
        id -> Integer,
        user_id -> Integer,
        identity_type -> Varchar,
        identifier -> Varchar,
        credential -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_role"]
    role (id) {
        id -> Integer,
        name -> Varchar,
        desc -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_user_role"]
    user_role (user_id, role_id) {
        user_id -> Integer,
        role_id -> Integer,
    }
}

table! {
    #[sql_name="lx_class"]
    class (id) {
        id -> Integer,
        name -> Varchar,
        nick_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_user_class"]
    user_class (user_id, class_id) {
        user_id -> Integer,
        class_id -> Integer,
    }
}

table! {
    #[sql_name="lx_activitie"]
    activities (id) {
        id -> Integer,
        title -> Varchar,
        active -> Bool,
        form -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_user_activitie"]
    user_activities (user_id, activities_id) {
        user_id -> Integer,
        activities_id -> Integer,
        data -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_message"]
    messages (id) {
        id -> Integer,
        title -> Varchar,
        content -> Varchar,
        msg_type -> Varchar,
    }
}