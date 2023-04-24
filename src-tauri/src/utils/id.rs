pub fn to_surreal_id(id: &str) -> &str {
    let mut db_id = id.split(":");
    db_id.nth(1).unwrap()
}
