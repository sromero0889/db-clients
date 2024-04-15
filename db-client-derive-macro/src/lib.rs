mod db_def;
mod input_parser;
mod output_parser;

use proc_macro_error::{proc_macro_error};
use quote::{ToTokens};
use syn::{parse_macro_input, DeriveInput};
use crate::db_def::DbType;


#[proc_macro_derive(SqliteEntity, attributes(col_cfg))]
#[proc_macro_error]
pub fn sqlite_entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let parent_field_attr = "col_cfg";
    // Get info from ast
    let db_table_def = input_parser::parse_struct_data(DbType::SqLite, parent_field_attr, &ident, &data);
    // Generate output
    let output: proc_macro2:: TokenStream = output_parser::generate_repository(&ident, &db_table_def);

    proc_macro::TokenStream::from(output)
}