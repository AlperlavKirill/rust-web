use chrono::{DateTime, Utc};
use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use crate::data::user::user::UserPublicInfo;
use crate::DB_URL;

pub(in crate::data::user) fn register_user(name: &str, email: &str, raw_password: &str) -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let password: String = hash_user_password(raw_password.to_string(), salt.clone());
    let _ = client.query_raw(
        "insert into users (id, name, email, register_date, password_hash, salt)\
        values (nextval('seq_user_id'), $1, $2, CURRENT_TIMESTAMP, $3, $4)",
        &[name, email, password, salt],
    )?;
    Ok(())
}

pub(in crate::data::user) fn get_user_public_info_from_db(id: String) -> Result<UserPublicInfo, PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    let id_num = id.parse::<i32>().expect("Input Id should be a number");
    let row = client.query_one(
        "select name, email, register_date from users where id = $1",
        &[&id_num],
    )?;
    let name: String = row.get("name");
    let email: String = row.get("email");
    let register_date: DateTime<Utc> = row.get("register_date");
    Ok(UserPublicInfo { name, email, register_date: register_date.to_string() })
}

pub(in crate::data::user) fn get_all_users() -> Result<Vec<UserPublicInfo>, PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    let rows = client.query("select name, email, register_date from users", &[])?;
    let mut all_users: Vec<UserPublicInfo> = Vec::new();
    for row in rows {
        let register_date: DateTime<Utc> = row.get("register_date");
        all_users.push(UserPublicInfo {
            name: row.get("name"),
            email: row.get("email"),
            register_date: register_date.to_string(),
        })
    }
    Ok(all_users)
}

pub(in crate::data::user) fn user_email_exists(email: &str) -> Result<bool, PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;

    let row = client.query_one(
        "select exists (select 1 from users where email = $1)",
        &[&email],
    )?;

    let exists: bool = row.get("exists");
    Ok(exists)
}


pub(in crate::data::user) fn check_user_exists(id: String) -> Result<bool, PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    let id_num = id.parse::<i32>().expect("Input ID should be a number");
    let row = client.query_one(
        "select exists (select 1 from users where id = $1)",
        &[&id_num],
    )?;
    let exists: bool = row.get("exists");
    Ok(exists)
}

// assume here that email exists
pub(in crate::data::user) fn check_login_data_correctness(email: &str, raw_password: &str) -> Result<bool, PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    let row = client.query_one(
        "select password_hash, salt from users where email = $1",
        &[&email],
    )?;

    let password_hash: String = row.get("password_hash");
    let salt: String = row.get("salt");
    Ok(hash_user_password(raw_password.to_string(), salt) == password_hash)
}

pub(in crate::data::user) fn delete_user(id: String) -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    let id_num = id.parse::<i32>().expect("Input ID should be a number");
    let _ = client.execute(
        "delete from users where id = $1",
        &[&id_num],
    )?;
    Ok(())
}

pub(in crate::data::user) fn update_user_info(id: String, name: String, email: String, raw_password: String) -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;
    let id_num = id.parse::<i32>().expect("Input ID should be a number");
    let salt_row = client.query_one(
        "select salt from users where id = $1",
        &[&id_num],
    )?;
    let salt: String = salt_row.get("salt");
    let password = hash_user_password(raw_password, salt);
    let _ = client.execute(
        "update users set name = $2, email = $3, password = $4 where id = $1",
        &[&id_num, &name, &email, &password],
    )?;
    Ok(())
}

//TODO! make method safer (temporary made just to test)
fn hash_user_password(raw_password: String, salt: String) -> String {
    raw_password + &salt
}
