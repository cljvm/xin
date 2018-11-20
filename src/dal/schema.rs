table! {
    #[sql_name="lx_activities"]
    activities (id) {
        id -> Int4,
        title -> Varchar,
        subject -> Varchar,
        fields -> Nullable<Jsonb>,
        content_raw -> Nullable<Varchar>,
        content_html -> Nullable<Varchar>,
        status -> Nullable<Bpchar>,
        view_count -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    #[sql_name="lx_activities_users"]
    activities_users (id) {
        id -> Int4,
        activitie_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        field1 -> Nullable<Varchar>,
        field2 -> Nullable<Varchar>,
        field3 -> Nullable<Varchar>,
        field4 -> Nullable<Varchar>,
        field5 -> Nullable<Varchar>,
        field6 -> Nullable<Varchar>,
        field7 -> Nullable<Varchar>,
        field8 -> Nullable<Varchar>,
        field9 -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    #[sql_name="lx_bulletins"]
    bulletins (id) {
        id -> Int4,
        title -> Varchar,
        subject -> Varchar,
        content_raw -> Nullable<Varchar>,
        content_html -> Nullable<Varchar>,
        status -> Nullable<Bpchar>,
        view_count -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    #[sql_name="lx_classes"]
    classes (id) {
        id -> Int4,
        name -> Varchar,
        alias_name -> Nullable<Varchar>,
        nick_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    #[sql_name="lx_classes_users"]
    classes_users (id) {
        id -> Int4,
        class_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
    }
}

table! {
    #[sql_name="lx_roles"]
    roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    #[sql_name="lx_roles_users"]
    roles_users (id) {
        id -> Int4,
        role_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
    }
}

table! {
    #[sql_name="lx_user_auths"]
    user_auths (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        identity_type -> Nullable<Bpchar>,
        identifier -> Nullable<Varchar>,
        credential -> Nullable<Varchar>,
        token -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    #[sql_name="lx_users"]
    users (id) {
        id -> Int4,
        name -> Varchar,
        nick_name -> Nullable<Varchar>,
        email -> Varchar,
        birthday -> Nullable<Date>,
        height -> Nullable<Int2>,
        weight -> Nullable<Int2>,
        status -> Nullable<Bpchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(activities_users -> activities (activitie_id));
joinable!(activities_users -> users (user_id));
joinable!(classes_users -> classes (class_id));
joinable!(classes_users -> users (user_id));
joinable!(roles_users -> roles (role_id));
joinable!(roles_users -> users (user_id));
joinable!(user_auths -> users (user_id));

allow_tables_to_appear_in_same_query!(
    activities,
    activities_users,
    bulletins,
    classes,
    classes_users,
    roles,
    roles_users,
    user_auths,
    users,
);