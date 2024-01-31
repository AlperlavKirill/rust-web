use crate::data::responses::{ALREADY_EXISTS, BAD_REQUEST, CREATED_RESPONSE, NOT_AUTHORIZED, NOT_FOUND, OK_RESPONSE};
use crate::data::user::service;
use crate::data::user::user::{UserLoginInfo, UserRegisterInfo};
use crate::INTERNAL_SERVER_ERROR;

pub(crate) fn handle_register_request(request: &str) -> (String, String) {
    match get_user_register_data_from_request_body(request) {
        Ok(user) => {
            let user_email = user.email;
            match service::user_email_exists(user_email.as_str()) {
                Ok(exists) => {
                    if exists {
                        (ALREADY_EXISTS.to_string(), "Email already exists".to_string())
                    } else {
                        let user_name = user.name;
                        let user_raw_password = user.password;

                        match service::register_user(&user_name, &user_email, &user_raw_password) {
                            Ok(_) => {
                                (CREATED_RESPONSE.to_string(), "success".to_string())
                            }
                            Err(e) => {
                                println!("Error registering a user! {}", e);
                                (INTERNAL_SERVER_ERROR.to_string(), "".to_string())
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error checking whether email exists or not! {}", e);
                    (INTERNAL_SERVER_ERROR.to_string(), "".to_string())
                }
            }
        }
        Err(e) => {
            println!("Error getting user from request body. {}", e);
            (BAD_REQUEST.to_string(), "".to_string())
        }
    }
}

pub(crate) fn handle_login_request(request: &str) -> (String, String) {
    match get_user_login_data_from_request_body(request) {
        Ok(user) => {
            let email = user.email;
            match service::user_email_exists(&email) {
                Ok(exists) => {
                    if !exists {
                        return (NOT_AUTHORIZED.to_string(), "Email or password is incorrect".to_string());
                    }
                }
                Err(e) => {
                    println!("Error checking whether email exists or not. {}", e);
                    return (INTERNAL_SERVER_ERROR.to_string(), "".to_string());
                }
            }
            let raw_password = user.password;
            match service::check_login_data_correctness(&email, &raw_password) {
                Ok(correct) => {
                    if correct {
                        // TODO! make session based authorization
                        return (OK_RESPONSE.to_string(), "success login".to_string());
                    }
                    (NOT_AUTHORIZED.to_string(), "Email or password is incorrect".to_string())
                }
                Err(e) => {
                    println!("Error checking login credentials. {}", e);
                    (INTERNAL_SERVER_ERROR.to_string(), "".to_string())
                }
            }
        }
        Err(e) => {
            println!("Error getting user from request body. {}", e);
            (BAD_REQUEST.to_string(), "".to_string())
        }
    }
}

pub fn handle_get_user_request(request: &str) -> (String, String) {
    let id = get_user_id_from_request(request);
    match service::check_user_exists(id.to_string()) {
        Ok(exists) => {
            if !exists {
                return (NOT_FOUND.to_string(), "No such user".to_string());
            }
            match service::get_user_public_info_from_db(id.to_string()) {
                Ok(user_public_info) => {
                    let response = serde_json::to_string(&user_public_info).unwrap_or_default();
                    (OK_RESPONSE.to_string(), response.to_string())
                }
                Err(e) => {
                    println!("Error getting user public data! {}", e);
                    (INTERNAL_SERVER_ERROR.to_string(), "".to_string())
                }
            }
        }
        Err(e) => {
            println!("Error checking if user exists. {}", e);
            (INTERNAL_SERVER_ERROR.to_string(), "".to_string())
        }
    }
}

pub fn handle_delete_user_request(request: &str) -> (String, String) {
    let id = get_user_id_from_request(request);
    match service::check_user_exists(id.to_string()) {
        Ok(exists) => {
            if !exists {
                return (NOT_FOUND.to_string(), "No such user".to_string())
            }
            match service::delete_user(id.to_string()) {
                Ok(_) => {
                    (OK_RESPONSE.to_string(), "user deleted".to_string())
                }
                Err(e) => {
                    println!("Error deleting user data! {}", e);
                    (INTERNAL_SERVER_ERROR.to_string(), "".to_string())
                }
            }
        }
        Err(e) => {
            println!("Error checking if user exists. {}", e);
            (INTERNAL_SERVER_ERROR.to_string(), "".to_string())
        }
    }
}

pub fn handle_get_all_users_request(_request: &str) -> (String, String) {
    match service::get_all_users() {
        Ok(users) => {
            let response = serde_json::to_string(&users).unwrap_or_default();
            (OK_RESPONSE.to_string(), response)
        }
        Err(err) => {
            println!("Error getting all users {}", err);
            (INTERNAL_SERVER_ERROR.to_string(), "".to_string())
        }
    }
}

pub fn handle_update_user_request(request: &str) -> (String, String) {
    let id = get_user_id_from_request(request);
    match get_user_register_data_from_request_body(request) {
        Ok(user_data) => {
            match service::check_user_exists(id.to_string()) {
                Ok(exists) => {
                    if exists {
                        match service::update_user_info(id.to_string(), user_data.name, user_data.email, user_data.password) {
                            Ok(_) => {
                                (OK_RESPONSE.to_owned(), "user updated".to_string())
                            }
                            Err(err) => {
                                println!("error updating user {}", err);
                                (INTERNAL_SERVER_ERROR.to_string(), "".to_string())
                            }
                        }
                    } else {
                        (NOT_FOUND.to_string(), "user not found".to_string())
                    }
                }
                Err(err) => {
                    println!("error checking if user exists {}", err);
                    (INTERNAL_SERVER_ERROR.to_owned(), "Error".to_string())
                }
            }
        }
        Err(err) => {
            println!("Error getting user from request body. {}", err);
            (INTERNAL_SERVER_ERROR.to_string(), "".to_string())
        }
    }

}

fn get_user_id_from_request(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

fn get_user_login_data_from_request_body(request: &str) -> Result<UserLoginInfo, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

fn get_user_register_data_from_request_body(request: &str) -> Result<UserRegisterInfo, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}
