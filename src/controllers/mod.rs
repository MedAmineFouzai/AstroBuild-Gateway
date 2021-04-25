extern crate hyper_multipart_rfc7578 ;
use hyper::{Client, Request};
use hyper_multipart_rfc7578::client::{self, multipart};
use actix_web::{get, http::StatusCode, HttpRequest, HttpResponse};
use async_graphql::ErrorExtensions;
use async_graphql::*;
use reqwest::{header};
use std::env;
use crate::middleware::error::UserCustomResponseError;
use serde_json::json;
use std::iter::FromIterator;
use std::io::Read;

mod schema;
use schema::{
    AddressModel, AuthResponseModel, PasswordModel, PhoneModel, UpdateUserInfo, UpdateUserPassword,EmailModel,
    UserId, UserInfo, UserLoginModel, UserModel, UserResponseModel,AddressInputModel,PhoneInputModel,PasswordInputModel,DeleteUserById,
    Category, CategoryResponseModel,File,Role

};



use load_dotenv::load_dotenv;

use self::schema::{};
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
                    .unwrap(),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let res = client
            .get( &format!("{}/api/v1/users/all",env!("BASE_URL")))
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                // let user: UserResponseModel = res
                // .json::<UserResponseModel>()
                // .await
                // .unwrap();
                Ok(res.json::<Vec<UserResponseModel>>().await.unwrap())
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "User Dosent Existe To Delete or Deactivate !"))),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User not ALLowed or Bad Authorization header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happend ! "))),
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
                    .unwrap(),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        // let data = ;
        let res = client
            .post(&format!("{}/api/v1/users/get",env!("BASE_URL")))
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
                    e.set("info", "User not ALLowed or Bad Authorization header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happend ! "))),
        }

        // let user: UserResponseModel = res
        //     .await
        //     .unwrap()
        //     .json::<UserResponseModel>()
        //     .await
        //     .unwrap();
        // user
    }

    // async fn current_token<'a>(&self, ctx: &'a Context<'_>) -> Option<&'a str> {
    //     ctx.data_opt::<MyToken>().map(|token| token.0.as_str())
    // }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {

    async fn signup(
        &self,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
        phone:PhoneInputModel,
        address:AddressInputModel,
        active: bool,
        role: Role,
    ) -> FieldResult<AuthResponseModel> {
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("{}/api/v1/users/auth/signup",env!("BASE_URL")))
            .json(&UserModel {
                email: email,
                password: password,
                first_name: first_name,
                last_name: last_name,
                phone: PhoneModel{
                    prefix:phone.prefix,
                    number: phone.number,
                },
                address: AddressModel{
                    place: address.place,
                    city: address.city,
                    zip: address.zip,
                    country: address.country,
                },
                active: active,
                role: match role {
                    Role::Admin => "Admin".to_string(),
                    Role::Client => "Client".to_string(),
                    Role::ProductOwner =>"ProductOwner".to_string(),
                    Role::Developer => "Developer".to_string(),
                }
            })
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let user: AuthResponseModel = res.json::<AuthResponseModel>().await.unwrap();
                Ok(user)
            },
            StatusCode::CONFLICT => Err(UserCustomResponseError::Conflict
                .extend_with(|_, e| e.set("info", "User Already Exist !"))),
            
            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "Bad Credentials Or User Dosent Existe !"))),

            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happend ! "))),
        }
    }

    async fn login(&self, email: String, password: String) -> FieldResult<AuthResponseModel> {
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("{}/api/v1/users/auth/login",env!("BASE_URL")))
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
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happend ! "))),
        }
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: String,password:String) -> FieldResult<UserResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap(),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let data: DeleteUserById = DeleteUserById { id: id ,password:password};
        let res = client
            .delete(&format!("{}/api/v1/users/delete",env!("BASE_URL")))
            .json(&data)
            .send()
            .await
            .unwrap();

        match res.status() {
            StatusCode::OK => {
                let user: UserResponseModel = res.json::<UserResponseModel>().await.unwrap();
                Ok(user)
            }

            StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
                .extend_with(|_, e| e.set("info", "User Dosent Existe To Delete or Deactivated !"))),
            StatusCode::FORBIDDEN => {
                Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
                    e.set("info", "User not ALLowed or Bad Authorization header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happend ! "))),
        }
    }

    async fn update_user_info(
        &self,
        ctx: &Context<'_>,
        id: String,
        email: String,
        phone:PhoneInputModel,
        address:AddressInputModel
    ) -> FieldResult<UserResponseModel> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(
                &ctx.data_opt::<MyToken>()
                    .map(|token| token.0.as_str())
                    .unwrap(),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let res = client
            .put(&format!("{}/api/v1/users/update/info",env!("BASE_URL")))
            .json(&UpdateUserInfo {
                id: id,
                user_info: UserInfo {
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
                    e.set("info", "User not ALLowed or Bad Authorization header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happend ! "))),
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
                    .unwrap(),
            )
            .unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        // let data = ;
        let res = client
            .put(&format!("{}/api/v1/users/update/password",env!("BASE_URL")))
            .json(&UpdateUserPassword {
                id: id,
                set_password: PasswordModel { old_password: password.old_password, new_password: password.new_password},
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
                    e.set("info", "User not ALLowed or Bad Authorization header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happend ! "))),
        }
    }

    async fn reset_user_password(
        &self,
        ctx: &Context<'_>,
        email:String
    ) -> FieldResult<UserResponseModel> {
        let mut headers = header::HeaderMap::new();
       
        let client = reqwest::Client::new();
        // let data = ;
        let res = client
            .post(&format!("{}/api/v1/users/auth/reset",env!("BASE_URL")))
            .json(&EmailModel{
                email:email
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
                    e.set("info", "User not ALLowed or Bad Authorization header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happend ! "))),
        }
    }


    
    async fn confirm_user_reset_password(
        &self,
        ctx: &Context<'_>,
        id: String,
        password:String
    ) -> FieldResult<UserResponseModel> {
        let mut headers = header::HeaderMap::new();
       
        let client = reqwest::Client::new();
        // let data = ;
        let res = client
            .put(&format!("{}/api/v1/users/auth/reset/confirm",env!("BASE_URL")))
            .json(&UpdateUserPassword {
                id: id,
                set_password: PasswordModel { old_password: "".to_string(), new_password: password},
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
                    e.set("info", "User not ALLowed or Bad Authorization header !")
                }))
            }
            _ => Err(UserCustomResponseError::ServerError
                .extend_with(|_, e| e.set("info", "Somthing Wrong Happend ! "))),
        }
    }

    //Categoru Mutations 
    
//     async fn add_category(&self, ctx: &Context<'_>, name: String, description: String
// //    ,image:Upload
// ) -> FieldResult<CategoryResponseModel> {
//         let mut headers = header::HeaderMap::new();
//         headers.insert(
//             header::AUTHORIZATION,
//             header::HeaderValue::from_str(
//                 &ctx.data_opt::<MyToken>()
//                     .map(|token| token.0.as_str())
//                     .unwrap(),
//             )
//             .unwrap(),
//         );
//         // let files=image.value(ctx).unwrap();
//         let mut form = multipart::Form::default();
//         form.add_text("name", name);
//         form.add_text("description", description);
//         form.add_file("image",file!()).expect("file to exist");
//         let mut req_builder = Request::post(&format!("{}/api/v1/builder/category/add",env!("BASE_URL")));
//         let req = form.set_body::<multipart::Body>(req_builder).unwrap();

//         if let Ok(_) = Client::builder().build_http().request(req).await{
//             println!("done...");
//             } else {
//             eprintln!("an error occurred");
//             };
         
       
        
        
        
//         // let form=multipart::Form::new()
//         // .text("name", name)
//         // .text("description", description);
      
//         // let file = File::open("cargo.toml")?;

//         // let client = reqwest::Client::builder()
//         //     .default_headers(headers)
//         //     .build()
//         //     .unwrap();
//         // let res=client.post(&format!("{}/api/v1/builder/category/add",env!("BASE_URL")))
//         // .
        
//         // body(file)
//         // .send().await.unwrap();
//         // println!("res: {:?}",res);
//         // // let data = ;
//         // let res = client
//         //     .put(&format!("{}/api/v1/users/update/password",env!("BASE_URL")))
//         //     .json(&UpdateUserPassword {
//         //         id: id,
//         //         set_password: PasswordModel { old_password: password.old_password, new_password: password.new_password},
//         //     })
//         //     .send()
//         //     .await
//         //     .unwrap();
//         // match res.status() {
//         //     StatusCode::OK => {
//         //         let user: UserResponseModel = res.json::<UserResponseModel>().await.unwrap();
//         //         Ok(user)
//         //     }

//         //     StatusCode::NOT_FOUND => Err(UserCustomResponseError::NotFound
//         //         .extend_with(|_, e| e.set("info", "User Dosent Existe To Update info !"))),
//         //     StatusCode::FORBIDDEN => {
//         //         Err(UserCustomResponseError::NotAllowed.extend_with(|_, e| {
//         //             e.set("info", "User not ALLowed or Bad Authorization header !")
//         //         }))
//         //     }
//         //     _ => Err(UserCustomResponseError::ServerError
//         //         .extend_with(|_, e| e.set("info", "Somthing Wrong Happend ! "))),
//         // }
//        Ok( CategoryResponseModel{
//             id: "sdqd".to_string(),
//             name: "qsdqsd".to_string(),
//             description: "qsdsqd".to_string(),
//             image: File{
//                 name:"sqdqsd".to_string(),
//                 src:"sqdqsd".to_string()
//             },
            
//         })

//     }




}
