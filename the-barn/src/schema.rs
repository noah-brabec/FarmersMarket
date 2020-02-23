table! {
    consumers (id) {
        id -> Uuid,
        name -> Varchar,
        address -> Json,
        #[sql_name = "type"]
        type_ -> Varchar,
        description -> Nullable<Text>,
        markets -> Nullable<Array<Uuid>>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
    }
}

table! {
    items (id) {
        id -> Uuid,
        name -> Varchar,
        price -> Money,
        price_unit -> Varchar,
        producer_id -> Nullable<Uuid>,
        description -> Nullable<Text>,
        post_date -> Nullable<Date>,
        tags -> Nullable<Array<Varchar>>,
    }
}

table! {
    markets (id) {
        id -> Uuid,
        name -> Varchar,
        address -> Json,
        description -> Nullable<Text>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
    }
}

table! {
    producers (id) {
        id -> Uuid,
        name -> Varchar,
        address -> Json,
        #[sql_name = "type"]
        type_ -> Varchar,
        markets -> Nullable<Array<Uuid>>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

table! {
    user_groups (group_id) {
        group_id -> Uuid,
        user_id -> Nullable<Uuid>,
        organization_id -> Uuid,
        role -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        first -> Varchar,
        last -> Varchar,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        password -> Varchar,
    }
}

joinable!(items -> producers (producer_id));
joinable!(user_groups -> users (user_id));

allow_tables_to_appear_in_same_query!(
    consumers,
    items,
    markets,
    producers,
    user_groups,
    users,
);
