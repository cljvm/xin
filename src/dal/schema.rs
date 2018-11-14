table! {
    #[sql_name="lx_users"]
    users (id) {
        id -> Integer,
        name -> Text,
        nick_name -> Nullable<Text>,
        email -> Nullable<Text>,
        birthday -> Nullable<Date>,
        height -> Nullable<SmallInt>,
        weight -> Nullable<SmallInt>,
        status -> SmallInt,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_user_auths"]
    user_auths (id) {
        id -> Integer,
        user_id -> Integer,
        identity_type -> Text,
        identifier -> Text,
        credential -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_roles"]
    roles (id) {
        id -> Integer,
        name -> Text,
        desc -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_roles_users"]
    roles_users (id) {
        id -> Integer,
        role_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    #[sql_name="lx_classes"]
    classes (id) {
        id -> Integer,
        name -> Text,
        nick_name -> Nullable<Text>,
        desc -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_classes_users"]
    users_classes (id) {
        id -> Integer,
        class_id -> Integer,
        user_id -> Integer,
    }
}

table! {
    #[sql_name="lx_activities"]
    activities (id) {
        id -> Integer,
        title -> Text,
        subject -> Text,
        fields -> Jsonb,
        content_raw -> Text,
        content_html -> Text,
        status -> SmallInt,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_activities_users"]
    activities_users (id) {
        id -> Integer,
        activitie_id -> Integer,
        user_id -> Integer,
        field1 -> Nullable<Text>,
        field2 -> Nullable<Text>,
        field3 -> Nullable<Text>,
        field4 -> Nullable<Text>,
        field5 -> Nullable<Text>,
        field6 -> Nullable<Text>,
        field7 -> Nullable<Text>,
        field8 -> Nullable<Text>,
        field9 -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    #[sql_name="lx_bulletins"]
    bulletins (id) {
        id -> Integer,
        subject -> Text,
        title -> Text,
        content_raw -> Text,
        content_html -> Text,
    }
}