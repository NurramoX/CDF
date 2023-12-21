use diesel::{AsChangeset, Insertable, Queryable};
use crate::schema::fast_forward;

#[derive(Insertable, AsChangeset)]
#[table_name = "fast_forward"]
pub struct KeyPath<'a> {
    pub key: &'a str,
    pub path: &'a str
}

#[derive(Debug, Queryable)]
pub struct KeyListItem {
    pub key: String
}
