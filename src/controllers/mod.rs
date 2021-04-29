use crate::{controllers::schema::SendAccountModel, middleware::error::UserCustomResponseError};
use actix_web::http::StatusCode;
use async_graphql::ErrorExtensions;
use async_graphql::*;
use reqwest::header;
use std::env;
// use serde_json::json;
// use std::iter::FromIterator;
// use std::io::Read;
use load_dotenv::load_dotenv;

mod schema;
use schema::{
    AddressInputModel, AddressModel, AuthResponseModel, CategoryResponseModel, DeleteUserById,
    EmailModel, FeatureResponseModel, FeatureToAnyModel, PasswordInputModel, PasswordModel,
    PhoneInputModel, PhoneModel, Role, SerlizedId, SpecificationInput, TemplateModel,
    TemplateResponseModel, UpdateUserInfo, UpdateUserPassword, UserId, UserInfo, UserLoginModel,
    UserModel, UserResponseModel,
};

load_dotenv!();
#[derive(Debug)]
pub struct MyToken(pub String);

pub type UserSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_all_users(&self, ctx: &Context<'_>) -> FieldResult<Vec<UserResponseModel>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client
            .get(&format!("{}/api/v1/users/all", env!("BASE_URL")))
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => Ok(res.json::<Vec<UserResponseModel>>().await.unwrap()),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "User Dosent Existe To Delete or Deactivate !"))),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User not ALLowed or Bad Authorization Header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_user_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<UserResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        // let data = ;
        let res = client
            .post(&format!("{}/api/v1/users/get", env!("BASE_URL")))
            .json(&UserId { id: id })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let user: UserResponseModel = res.json::<UserResponseModel>().await.unwrap();
                Ok(user)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "User Dosent Existe To Delete or Deactivate !"))),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User not ALLowed or Bad Authorization Header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_category_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<CategoryResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        // let data = ;
        let res = client
            .post(&format!("{}/api/v1/builder/category/get", env!("BASE_URL")))
            .json(&SerlizedId { id: id })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let category: CategoryResponseModel =
                    res.json::<CategoryResponseModel>().await.unwrap();
                Ok(category)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Category not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", " Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_feature_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<FeatureResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        // let data = ;
        let res = client
            .post(&format!("{}/api/v1/builder/feature/get", env!("BASE_URL")))
            .json(&SerlizedId { id: id })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let feature: FeatureResponseModel =
                    res.json::<FeatureResponseModel>().await.unwrap();
                Ok(feature)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Feature not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", " Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_template_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<TemplateResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let res = client
            .post(&format!("{}/api/v1/builder/template/get", env!("BASE_URL")))
            .json(&SerlizedId { id: id })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let template: TemplateResponseModel =
                    res.json::<TemplateResponseModel>().await.unwrap();
                Ok(template)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Template not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", " Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_all_categories(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<CategoryResponseModel>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client
            .get(&format!("{}/api/v1/builder/category/all", env!("BASE_URL")))
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => Ok(res.json::<Vec<CategoryResponseModel>>().await.unwrap()),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Category not Found !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_all_features(&self, ctx: &Context<'_>) -> FieldResult<Vec<FeatureResponseModel>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client
            .get(&format!("{}/api/v1/builder/feature/all", env!("BASE_URL")))
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => Ok(res.json::<Vec<FeatureResponseModel>>().await.unwrap()),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Feature not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn get_all_templates(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<TemplateResponseModel>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client
            .get(&format!("{}/api/v1/builder/template/all", env!("BASE_URL")))
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => Ok(res.json::<Vec<TemplateResponseModel>>().await.unwrap()),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Feature not Found!"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    // async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
    //     ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    // }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn signup(&self, email: String, password: String) -> FieldResult<AuthResponseModel> {
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("{}/api/v1/users/auth/signup", env!("BASE_URL")))
            .json(&UserModel {
                email: email,
                password: password,
                first_name: "".to_string(),
                last_name: "".to_string(),
                phone: PhoneModel {
                    prefix: "".to_string(),
                    number: "".to_string(),
                },
                address: AddressModel {
                    place: "".to_string(),
                    city: "".to_string(),
                    zip: "".to_string(),
                    country: "".to_string(),
                },
                active: true,
                role: "Client".to_string(),
            })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let user: AuthResponseModel = res.json::<AuthResponseModel>().await.unwrap();
                Ok(user)
            }
            StatusCode::CONFLICT => Err(UserCustomResponseError::Conflict
                .extend_with(|_, e| e.set("info", "User Already Exist !"))),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Bad Credentials Or User Dosent Existe !"))),

            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn create_user(
        &self,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
        phone: PhoneInputModel,
        address: AddressInputModel,
        active: bool,
        role: Role,
    ) -> FieldResult<UserResponseModel> {
        let email_model = &SendAccountModel {
            email: email.clone(),
            password: password.clone(),
            role: match role {
                Role::Admin => "Admin".to_string(),
                Role::Client => "Client".to_string(),
                Role::ProductOwner => "ProductOwner".to_string(),
                Role::Developer => "Developer".to_string(),
            },
        };
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("{}/api/v1/users/auth/signup", env!("BASE_URL")))
            .json(&UserModel {
                email: email,
                password: password,
                first_name: first_name,
                last_name: last_name,
                phone: PhoneModel {
                    prefix: phone.prefix,
                    number: phone.number,
                },
                address: AddressModel {
                    place: address.place,
                    city: address.city,
                    zip: address.zip,
                    country: address.country,
                },
                active: active,
                role: match role {
                    Role::Admin => "Admin".to_string(),
                    Role::Client => "Client".to_string(),
                    Role::ProductOwner => "ProductOwner".to_string(),
                    Role::Developer => "Developer".to_string(),
                },
            })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let client = reqwest::Client::new();
                let res = client
                    .post(&format!(
                        "{}/api/v1/users/auth/send/account",
                        env!("BASE_URL")
                    ))
                    .json(email_model)
                    .send()
                    .await
                    .unwrap();
                match res.status() {
                    StatusCode::OK => {
                        let user: UserResponseModel =
                            res.json::<UserResponseModel>().await.unwrap();
                        Ok(user)
                    }
                    _ => Err(UserCustomResponseError::ServerError
                        .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
                }
            }
            StatusCode::CONFLICT => Err(UserCustomResponseError::Conflict
                .extend_with(|_, e| e.set("info", "User Already Exist !"))),

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Bad Credentials Or User Dosent Existe !"))),

            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn login(&self, email: String, password: String) -> FieldResult<AuthResponseModel> {
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("{}/api/v1/users/auth/login", env!("BASE_URL")))
            .json(&UserLoginModel {
                email: email,
                password: password,
            })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let user: AuthResponseModel = res.json::<AuthResponseModel>().await.unwrap();
                Ok(user)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Bad Credentials Or User Dosent Existe !"))),

            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn delete_user(
        &self,
        ctx: &Context<'_>,
        id: String,
        password: String,
    ) -> FieldResult<UserResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let data: DeleteUserById = DeleteUserById {
            id: id,
            password: password,
        };
        let res = client
            .delete(&format!("{}/api/v1/users/delete", env!("BASE_URL")))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let user: UserResponseModel = res.json::<UserResponseModel>().await.unwrap();
                Ok(user)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound.extend_with(|_, e| {
                e.set("info", "User Dosent Existe To Delete or Deactivated !")
            })),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User not ALLowed or Bad Authorization Header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn update_user_info(
        &self,
        ctx: &Context<'_>,
        id: String,
        first_name: String,
        last_name: String,
        email: String,
        phone: PhoneInputModel,
        address: AddressInputModel,
    ) -> FieldResult<UserResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let res = client
            .put(&format!("{}/api/v1/users/update/info", env!("BASE_URL")))
            .json(&UpdateUserInfo {
                id: id,
                user_info: UserInfo {
                    first_name: first_name,
                    last_name: last_name,
                    email: email,
                    phone: PhoneModel {
                        prefix: phone.prefix,
                        number: phone.number,
                    },
                    address: AddressModel {
                        place: address.place,
                        city: address.city,
                        zip: address.zip,
                        country: address.country,
                    },
                },
            })
            .send()
            .await
            .unwrap();
        match res.status() {
            StatusCode::OK => {
                let user: UserResponseModel = res.json::<UserResponseModel>().await.unwrap();
                Ok(user)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "User Dosent Existe To Update info !"))),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User not ALLowed or Bad Authorization Header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn update_user_password(
        &self,
        ctx: &Context<'_>,
        id: String,
        password: PasswordInputModel,
    ) -> FieldResult<UserResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        // let data = ;
        let res = client
            .put(&format!(
                "{}/api/v1/users/update/password",
                env!("BASE_URL")
            ))
            .json(&UpdateUserPassword {
                id: id,
                set_password: PasswordModel {
                    old_password: password.old_password,
                    new_password: password.new_password,
                },
            })
            .send()
            .await
            .unwrap();
        match res.status() {
            StatusCode::OK => {
                let user: UserResponseModel = res.json::<UserResponseModel>().await.unwrap();
                Ok(user)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "User Dosent Existe To Update info !"))),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User not ALLowed or Bad Authorization Header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn reset_user_password(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> FieldResult<UserResponseModel> {
        let client = reqwest::Client::new();
        // let data = ;
        let res = client
            .post(&format!("{}/api/v1/users/auth/reset", env!("BASE_URL")))
            .json(&EmailModel { email: email })
            .send()
            .await
            .unwrap();
        match res.status() {
            StatusCode::OK => {
                let user: UserResponseModel = res.json::<UserResponseModel>().await.unwrap();
                Ok(user)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "User Dosent Existe To Update info !"))),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User not ALLowed or Bad Authorization Header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn confirm_user_reset_password(
        &self,
        ctx: &Context<'_>,
        id: String,
        password: String,
    ) -> FieldResult<UserResponseModel> {
        let client = reqwest::Client::new();
        // let data = ;
        let res = client
            .put(&format!(
                "{}/api/v1/users/auth/reset/confirm",
                env!("BASE_URL")
            ))
            .json(&UpdateUserPassword {
                id: id,
                set_password: PasswordModel {
                    old_password: "".to_string(),
                    new_password: password,
                },
            })
            .send()
            .await
            .unwrap();
        match res.status() {
            StatusCode::OK => {
                let user: UserResponseModel = res.json::<UserResponseModel>().await.unwrap();
                Ok(user)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "User Dosent Existe To Reset Password !"))),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User not ALLowed or Bad Authorization Header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    //Categoru Mutations
    async fn delete_category(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<CategoryResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let data: SerlizedId = SerlizedId { id: id };
        let res = client
            .delete(&format!(
                "{}/api/v1/builder/category/delete",
                env!("BASE_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let category: CategoryResponseModel =
                    res.json::<CategoryResponseModel>().await.unwrap();
                Ok(category)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Category Dosent Existe To Delete !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn delete_feature(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> FieldResult<FeatureResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let data: SerlizedId = SerlizedId { id: id };
        let res = client
            .delete(&format!(
                "{}/api/v1/builder/feature/delete",
                env!("BASE_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let feature: FeatureResponseModel =
                    res.json::<FeatureResponseModel>().await.unwrap();
                Ok(feature)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Feature Dosent Existe To Delete !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn delete_template(&self, ctx: &Context<'_>, id: String) -> FieldResult<TemplateModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let data: SerlizedId = SerlizedId { id: id };
        let res = client
            .delete(&format!(
                "{}/api/v1/builder/template/delete",
                env!("BASE_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let template: TemplateModel = res.json::<TemplateModel>().await.unwrap();
                Ok(template)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Feature Dosent Existe To Delete !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn add_template_feature(
        &self,
        ctx: &Context<'_>,
        id: String,
        features_id: Vec<String>,
    ) -> FieldResult<TemplateResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let data: FeatureToAnyModel = FeatureToAnyModel {
            id: id,
            features_id: features_id,
        };
        let res = client
            .post(&format!(
                "{}/api/v1/builder/template/feature/add",
                env!("BASE_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let template: TemplateResponseModel =
                    res.json::<TemplateResponseModel>().await.unwrap();
                Ok(template)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Category Dosent Existe To Delete !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn delete_template_feature(
        &self,
        ctx: &Context<'_>,
        id: String,
        featuer_id: String,
    ) -> FieldResult<TemplateResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let data: FeatureToAnyModel = FeatureToAnyModel {
            id: id,
            features_id: vec![featuer_id],
        };
        let res = client
            .delete(&format!(
                "{}/api/v1/builder/template/feature/delete",
                env!("BASE_URL")
            ))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let template: TemplateResponseModel =
                    res.json::<TemplateResponseModel>().await.unwrap();
                Ok(template)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Category Dosent Existe To Delete !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }

    async fn add_template_specification(
        &self,
        ctx: &Context<'_>,
        id: String,
        specification: SpecificationInput,
    ) -> FieldResult<TemplateResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap_or("Authorization "),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let form = reqwest::multipart::Form::new()
            .text("purpose", specification.introduction.purpose)
            .text(
                "document_conventions",
                specification.introduction.document_conventions,
            )
            .text(
                "intended_audience",
                specification.introduction.intended_audience,
            )
            .text("project_scope", specification.introduction.project_scope)
            .text("perspective", specification.overall_description.perspective)
            .text(
                "user_characteristics",
                specification.overall_description.user_characteristics,
            )
            .text(
                "operating_environment",
                specification.overall_description.operating_environment,
            )
            .text(
                "design_implementation_constraints",
                specification
                    .overall_description
                    .design_implementation_constraints,
            )
            .text(
                "user_documentation",
                specification.overall_description.user_documentation,
            )
            .text(
                "assemptions_dependencies",
                specification.overall_description.assemptions_dependencies,
            )
            .text(
                "performance_requirements",
                specification
                    .non_functional_requirements
                    .performance_requirements,
            )
            .text(
                "safety_requirements",
                specification
                    .non_functional_requirements
                    .safety_requirements,
            )
            .text(
                "security_requirements",
                specification
                    .non_functional_requirements
                    .security_requirements,
            )
            .text(
                "software_quality_attributes",
                specification
                    .non_functional_requirements
                    .software_quality_attributes,
            )
            .text("other_requirements", specification.other_requirements)
            .text("glossary", specification.glossary)
            .text("analysis_models", specification.analysis_models)
            .text("issues_list", specification.issues_list)
            .text("id", id);

        let res = client
            .put(&format!(
                "{}/api/v1/builder/template/specification/add",
                env!("BASE_URL")
            ))
            .multipart(form)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let template: TemplateResponseModel =
                    res.json::<TemplateResponseModel>().await.unwrap();
                Ok(template)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Category Dosent Existe To Delete !"))),
            StatusCode::FORBIDDEN => Err(UserCustomResponseError::NotAllowed
                .extend_with(|_, e| e.set("info", "Bad Authorization Header !"))),
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happenend ! "))),
        }
    }
}
