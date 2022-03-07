table! {
    dwz (id) {
        id -> Unsigned<Integer>,
        short_url -> Varchar,
        redirect_url -> Varchar,
        valid_time -> Datetime,
        create_time -> Datetime,
    }
}
