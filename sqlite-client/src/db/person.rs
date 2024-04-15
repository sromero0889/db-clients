use db_client_derive_macro::SqliteEntity;
#[derive(Default, SqliteEntity, Debug)]
struct Person {
    #[col_cfg(id)]
    id: u32,
    #[col_cfg(unique)]
    email: String,
    name: String,
    #[col_cfg(unique, index)]
    passport: String,
    age: Option<usize>
}