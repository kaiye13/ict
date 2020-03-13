use super::schema::logs;
use crate::log::IgnoreNoneFieldsUpdateLog;
use crate::log::Log;
use crate::log::NewLog;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn create(log: NewLog, connection: &MysqlConnection) -> Log {
    diesel::insert_into(logs::table)
        .values(&log)
        .execute(connection)
        .expect("Error creating new log");

    logs::table
        .order(logs::id.desc())
        .first(connection)
        .unwrap()
}

pub fn read(connection: &MysqlConnection) -> Vec<Log> {
    logs::table
        .order(logs::id.asc())
        .load::<Log>(connection)
        .unwrap()
}

pub fn update(id: i32, log: IgnoreNoneFieldsUpdateLog, connection: &MysqlConnection) -> bool {
    diesel::update(logs::table.find(id))
        .set(&log)
        .execute(connection)
        .is_ok()
}

pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
    diesel::delete(logs::table.find(id))
        .execute(connection)
        .is_ok()
}
