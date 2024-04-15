use proc_macro_error::abort;

#[derive(PartialEq, Eq)]
pub enum DbColumnAttributes {
    Id,
    Index,
    Unique
}

impl TryInto<DbColumnAttributes> for &str {
    type Error = ();

    fn try_into(self) -> Result<DbColumnAttributes, Self::Error> {
        match self {
            "id" => Ok(DbColumnAttributes::Id),
            "index" => Ok(DbColumnAttributes::Index),
            "unique" => Ok(DbColumnAttributes::Unique),
            _ => Err(()),
        }
    }
}

pub enum DbType {
    SqLite,
    // todo(): Add here more db types to support other dbs
}

pub struct DbColumnDef {
    pub name: String,
    pub c_type: String,
    pub is_pk: bool,
    pub is_index: bool,
    pub is_unique: bool,
    pub is_not_null: bool
}



impl DbColumnDef {
    pub fn get_db_column_type(&self, db_type: &DbType) -> &str {
        match db_type {
            DbType::SqLite => {
                match self.c_type.as_str() {
                    "bool" | "usize" | "u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32" | "i64" | "i128" => "INTEGER",
                    "f32" | "f64" => "REAL",
                    "String" | "str" => "TEXT",
                    // "" => "BLOB",
                    _ => abort!(self.c_type, "Not available")
                }
            }
        }
    }
}

pub struct DbTableDef {
    pub name: String,
    pub columns: Vec<DbColumnDef>,
    pub indexes: Vec<String>,
    pub unique_indexes: Vec<String>,
    pub pks: Vec<String>,
    pub db_type: DbType
}

impl DbTableDef {
    fn as_sqlite_create_query(&self) -> String {
        let table_name = self.name.as_str();
        let columns_def: Vec<String> = self.columns.iter().map(|c| {
            let col_def: String = c.name.clone();
            let col_type = c.get_db_column_type(&self.db_type);
            let mut col_def: String = format!("{col_def} {col_type}");
            if c.is_unique {
                col_def = format!("{col_def} UNIQUE");
            };
            if c.is_not_null {
                col_def = format!("{col_def} NOT NULL");
            };

            col_def
        }).collect();
        let columns_def: String = columns_def.join(", ");


        let mut create_table = if self.pks.is_empty() {
            format!("CREATE TABLE IF NOT EXISTS {table_name} ( {columns_def} );")
        } else {
            let pks_def: Vec<String> = self.columns.iter().filter(|c| c.is_pk ).map(|c| c.name.clone() ).collect();
            let pks_def = pks_def.join(",");
            format!("CREATE TABLE IF NOT EXISTS {table_name} ( {columns_def}, PRIMARY KEY ({pks_def}));")
        };
        if !self.indexes.is_empty() {
            let create_indexes: Vec<String> = self.indexes.iter().map(|i| format!("CREATE INDEX idx_{i} ON {table_name} ({i})") ).collect();
            let create_indexes = create_indexes.join("\n");

            create_table = format!("{create_table}\n {create_indexes}");
        }
        if !self.unique_indexes.is_empty() {
            let create_unique_indexes: Vec<String> = self.unique_indexes.iter().map(|i| format!("CREATE UNIQUE INDEX idx_{i} ON {table_name} ({i});") ).collect();
            let create_unique_indexes = create_unique_indexes.join("\n");

            create_table = format!("{create_table}\n {create_unique_indexes}");
        }

        create_table
    }
    pub fn as_create_query(&self) -> String {
        match self.db_type {
            DbType::SqLite => self.as_sqlite_create_query()
        }
    }
}