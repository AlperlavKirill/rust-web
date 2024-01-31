extern crate chrono;

use postgres::{Client, NoTls};
use postgres::Error as PostgresError;

use crate::DB_URL;

#[derive(Serialize, Deserialize)]
pub(in crate::data::user) struct UserPublicInfo {
    pub(in crate::data::user) name: String,
    pub(in crate::data::user) email: String,
    pub(in crate::data::user) register_date: String,
}

#[derive(Serialize, Deserialize)]
pub(in crate::data::user) struct UserLoginInfo {
    pub(in crate::data::user) email: String,
    pub(in crate::data::user) password: String,
}

#[derive(Serialize, Deserialize)]
pub(in crate::data::user) struct UserRegisterInfo {
    pub(in crate::data::user) name: String,
    pub(in crate::data::user) email: String,
    pub(in crate::data::user) password: String,
}

pub(crate) fn setup_users_db() -> Result<(), PostgresError> {
    let mut client = Client::connect(DB_URL, NoTls)?;

    client.batch_execute(
        "
create table if not exists users
(
    id            serial primary key,
    name          varchar     not null,
    email         varchar     not null unique,
    register_date timestamptz not null,
    password_hash varchar     not null,
    salt          varchar     not null
);
"
    )?;
    client.batch_execute(
        "create sequence if not exists seq_user_id start with 1 increment by 1;"
    )?;
    Ok(())
}


