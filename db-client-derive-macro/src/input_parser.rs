use proc_macro2::TokenTree;
use proc_macro_error::abort;
use syn::{AttrStyle, DataStruct, Fields, GenericArgument, Meta, PathArguments, Type, TypePath, TypeReference};
use crate::db_def::{DbColumnAttributes, DbColumnDef, DbTableDef, DbType};


fn filter_allowed_attributes(token: TokenTree) -> Option<DbColumnAttributes> {
    let str_token = token.to_string();
    let str_token = str_token.as_str();
    match str_token.try_into() {
        Ok(col_attr) => Some(col_attr),
        Err(_) => None
    }
}

fn parse_field_attribute(parent_field_attr: &str,field_attr: &syn::Attribute) -> Option<Vec<DbColumnAttributes>> {
    let attr_ident = field_attr.path().get_ident().unwrap();
    let syn::Attribute { meta, style, .. } = field_attr;
    if attr_ident.to_string().as_str() == parent_field_attr {
        match (style, meta) {
            (AttrStyle::Outer, Meta::List(ml)) => {
                let i: Vec<DbColumnAttributes> = ml.clone().tokens.into_iter().filter_map(|t| filter_allowed_attributes(t)).collect();
                Some(i)
            },
            _ => None
        }
    } else {
        None
    }

}

fn parse_field(parent_field_attr: &str, field: &syn::Field) -> DbColumnDef {
    let f_ident = field.ident.clone().expect("Field ident is required").to_string();
    let (c_type, is_not_null): (String, bool) = match field.ty {
        // Type::Array(_) => {} todo()! next version
        Type::Reference(TypeReference { ref elem, ..}) => {
            if let Type::Path(TypePath { ref path, .. }) = *elem.clone() {
                let last_segment = path.segments.last().expect(format!("type of {} not supported", f_ident.as_str()).as_str());
                let f_type_ident = last_segment.ident.to_string();
                if f_type_ident == "str" {
                    (f_type_ident, true)
                } else { abort!(field.ty, "Type Not supported") }
            } else { abort!(field.ty, "Type Not supported") }

        },
        Type::Path(TypePath { ref path, .. }) => {
            let last_segment = path.segments.last().expect(format!("type of {} not supported", f_ident.as_str()).as_str());
            let f_type_ident = last_segment.ident.to_string();
            if f_type_ident.eq("Option") || f_type_ident.ends_with("::Option") {
                match &last_segment.arguments {
                    PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{ref args, ..})=> {
                        if args.len() > 1 {
                            abort!(field.ty, "Type Not supported")
                        }
                        match args.first().expect("type of {f_ident} not supported") {

                            GenericArgument::Type(Type::Path(TypePath { ref path, .. })) => {

                                let inner_last_segment = path.segments.last().expect(format!("type of {} not supported", f_ident.as_str()).as_str());
                                (inner_last_segment.ident.to_string(), false)
                            }

                            _ => abort!(field.ty, "Type Not supported")
                        }
                    }
                    _ => abort!(field.ty, "Type Not supported")
                }
            } else {
                (f_type_ident, true)
            }

        }
        _ => { abort!(field.ty, "Type Not supported")}
    };

    let f_attrs: Vec<DbColumnAttributes> = field.attrs.iter().filter_map(|fa| parse_field_attribute(parent_field_attr, fa)).flatten().collect();
    let is_pk = f_attrs.iter().any(|a| a == &DbColumnAttributes::Id);
    let is_index = f_attrs.iter().any(|a| a == &DbColumnAttributes::Index);
    let is_unique = f_attrs.iter().any(|a| a == &DbColumnAttributes::Unique);
    DbColumnDef{
        name: f_ident,
        c_type,
        is_pk,
        is_index,
        is_unique,
        is_not_null,
    }
}


pub fn parse_struct_data(db_type: DbType, parent_field_attr: &str, ident: &syn::Ident, input_data: &syn::Data) -> DbTableDef {
    match input_data {
        syn::Data::Struct(DataStruct { fields: Fields::Named( ref named_fields), ..}) => {
            let columns: Vec<DbColumnDef> = named_fields.named.iter().map(|f| parse_field(parent_field_attr, f)).collect();
            let indexes: Vec<String> = columns.iter().filter(|c| c.is_index && !c.is_unique ).map(|c| c.name.clone()).collect();
            let unique_indexes: Vec<String> = columns.iter().filter(|c| c.is_index && c.is_unique ).map(|c| c.name.clone()).collect();
            let pks: Vec<String> = columns.iter().filter(|c| c.is_pk ).map(|c| c.name.clone()).collect();
            let name = ident.to_string().to_lowercase();
            DbTableDef {
                name,
                columns,
                indexes,
                unique_indexes,
                pks,
                db_type,
            }
        },
        _  => abort!(ident.clone(), "Only available for structs with named fields")
    }
}