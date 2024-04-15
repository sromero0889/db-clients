use db_client_derive_macro::SqliteEntity;
#[derive(Default, SqliteEntity, Debug)]
struct Car {
    #[col_cfg(id)]
    id: u32,
    #[col_cfg(unique)]
    reference_code: String
}