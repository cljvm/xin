table! {
    #[sql_name="lx_user"]
    user (id) {
        id -> Unsigned<Integer>,
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
    #[sql_name="lx_role"]
    role (id) {
        id -> Unsigned<Integer>,
        name -> Varchar,
        desc -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_user_role"]
    user_role (user_id, role_id) {
        user_id -> Unsigned<Integer>,
        role_id -> Unsigned<Integer>,
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
    #[sql_name="lx_activitie"]
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
    #[sql_name="lx_user_activitie"]
    user_activities (user_id, activities_id) {
        user_id -> Unsigned<Integer>,
        activities_id -> Unsigned<Integer>,
        data -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_message"]
    messages (id) {
        id -> Unsigned<Integer>,
        title -> Varchar,
        content -> Varchar,
        msg_type -> Varchar,
    }
}