table! {
    user_info (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        salt -> Text,
        cryptpassword -> Text,
    }
}
