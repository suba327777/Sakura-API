// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int8,
        #[max_length = 255]
        username -> Varchar,
        grade -> Int4,
        expiration_date -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    card (id) {
        id -> Int8,
        account_id -> Int8,
        #[max_length = 255]
        card_name -> Varchar,
        card_number -> Bytea,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    door (device_id) {
        #[max_length = 255]
        device_id -> Varchar,
        door_state -> Nullable<Bool>,
        door_switch_state -> Nullable<Bool>,
    }
}

diesel::joinable!(card -> account (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    card,
    door,
);
