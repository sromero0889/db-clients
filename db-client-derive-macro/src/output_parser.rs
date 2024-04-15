use quote::quote;
use crate::db_def::DbTableDef;

pub fn generate_repository(ident: &syn::Ident, table_def: &DbTableDef) -> proc_macro2::TokenStream {
    let repo_struct_name: proc_macro2::TokenStream = format!("{}Repository", ident.to_string()).parse().expect("Cannot construct the Repository struct name");
    // let repo_struct_name = quote!(#repo_struct_name);
    // let create_query = format!("\"{}\"",table_def.as_create_query());
    let create_query = table_def.as_create_query();

    quote! {
        pub struct #repo_struct_name {}

        impl #repo_struct_name {
            pub fn get_create_table_query() -> &'static str {
                #create_query
            }
            pub fn create_table(connection: &rusqlite::Connection) -> rusqlite::Result<usize> {
                connection.execute(Self::get_create_table_query(), ())
            }
        }
    }
}