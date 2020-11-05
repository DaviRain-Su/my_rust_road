table! {
    use_path (id) {
        id -> Integer,
        prenum -> Integer,
        fname -> Text,
        ftype -> Text,
        pfname -> Text,
        md5 -> Text,
        fsize -> Integer,
        vfname -> Text,
    }
}

table! {
    user_info (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        salt -> Text,
        cryptpassword -> Text,
    }
}

table! {
    user_request (id) {
        id -> Integer,
        username -> Text,
        request -> Text,
        time -> Text,
        token -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    use_path,
    user_info,
    user_request,
);
