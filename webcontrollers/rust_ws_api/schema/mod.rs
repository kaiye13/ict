table! {
    logs (id) {
        id -> Integer,
        app_id -> Varchar,
        dev_id -> Varchar,
        time -> Varchar,
        payload -> Varchar,
    }
}

table! {
    requests (id) {
        id -> Integer,
        api_key -> Varchar,
        log -> Varchar,
    }
}
