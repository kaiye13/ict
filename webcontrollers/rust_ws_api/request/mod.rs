use super::schema::requests;
use crate::log::Log;

#[table_name = "requests"]
#[derive(Serialize, Deserialize, Identifiable, Queryable)]
pub struct Request {
    pub id: i32,
    pub api_key: String,
    pub log: Log,
}

#[table_name = "requests"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct NewRequest<'a> {
    pub api_key: String,
    pub log: Log,
}

#[table_name = "requests"]
#[derive(Serialize, Deserialize, AsChangeset)]
pub struct IgnoreNoneFieldsUpdateRequest<'a> {
    pub api_key: &'a str,
}
