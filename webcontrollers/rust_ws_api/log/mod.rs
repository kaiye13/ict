use super::schema::logs;

#[table_name = "logs"]
#[derive(Serialize, Deserialize, Identifiable, Queryable)]
pub struct Log {
    pub id: i32,
    pub app_id: String,
    pub dev_id: String,
    pub time: String,
    pub payload: String,
}

#[table_name = "logs"]
#[derive(Serialize, Deserialize, Insertable)]
pub struct NewLog<'a> {
    pub app_id: &'a str,
    pub dev_id: &'a str,
    pub time: &'a str,
    pub payload: &'a str,
}

#[table_name = "logs"]
#[derive(Serialize, Deserialize, AsChangeset)]
pub struct IgnoreNoneFieldsUpdateLog<'a> {
    pub app_id: &'a str,
    pub dev_id: &'a str,
    pub time: &'a str,
    pub payload: &'a str,
}
