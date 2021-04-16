use async_graphql::{InputObject,SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, SimpleObject, Deserialize)]
pub struct UserId {
    pub id: String,
}
#[derive(Debug, Serialize, SimpleObject, Deserialize, Clone)]
pub struct PhoneModel {
    pub prefix: String,
    pub number: String,
}

#[derive(Debug, Serialize, InputObject, Deserialize, Clone)]
pub struct PhoneInputModel {
    pub prefix: String,
    pub number: String,
}


#[derive(Debug, Clone, SimpleObject, Deserialize, Serialize)]
pub struct AddressModel {
    pub place: String,
    pub city: String,
    pub zip: String,
    pub country: String,
}


#[derive(Debug, Clone, InputObject, Deserialize, Serialize)]
pub struct AddressInputModel {
    pub place: String,
    pub city: String,
    pub zip: String,
    pub country: String,
}

#[derive(Debug, Clone, SimpleObject, Deserialize, Serialize)]
pub struct UserModel {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: PhoneModel,
    pub address: AddressModel,
    pub active: bool,
    pub role: String,
}

#[derive(Debug, Clone, SimpleObject, Deserialize, Serialize)]
pub struct UserResponseModel {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: PhoneModel,
    pub address: AddressModel,
    pub active: bool,
    pub role: String,
}

#[derive(Debug, InputObject, Serialize, Deserialize, Clone)]
pub struct PasswordInputModel {
    pub old_password:String,
    pub new_password: String,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize, Clone)]
pub struct PasswordModel {
    pub old_password:String,
    pub new_password: String,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub email: String,
    pub phone: PhoneModel,
    pub address: AddressModel,
}

// #[derive(Debug,Serialize, Deserialize, Clone)]
// pub enum UpdateObject {
//     UpdateUserInfo {
//         id: String,
//         user_info: UserInfo,
//     },
//     UpdatePassword {
//         id: String,
//         set_password: PasswordModel,
//     },
// }

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct UpdateUserInfo {
    pub id: String,
    pub user_info: UserInfo,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct UpdateUserPassword {
    pub id: String,
    pub set_password: PasswordModel,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct UserLoginModel {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailModel {
    pub email: String,
}

#[derive(Debug, Clone, SimpleObject, Deserialize, Serialize)]
pub struct AuthResponseModel {
    pub user: UserResponseModel,
    pub token: String,
}
