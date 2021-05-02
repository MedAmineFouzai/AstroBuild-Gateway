use async_graphql::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, SimpleObject, Deserialize)]
pub struct UserId {
    pub id: String,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Role {
    #[graphql(name = "Admin")]
    Admin,
    #[graphql(name = "Client")]
    Client,
    #[graphql(name = "ProductOwner")]
    ProductOwner,
    #[graphql(name = "Developer")]
    Developer,
}

#[derive(Debug, Serialize, SimpleObject, Deserialize)]
pub struct DeleteUserById {
    pub id: String,
    pub password: String,
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

#[derive(Debug, Serialize, SimpleObject, Deserialize, Clone)]
pub struct CountryPrefixModel {
    pub country:String,
    pub prefix: String,
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

    pub role: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SerlizedId {
    pub id: String,
}

#[derive(Debug, InputObject, Serialize, Deserialize, Clone)]
pub struct PasswordInputModel {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize, Clone)]
pub struct PasswordModel {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: PhoneModel,
    pub address: AddressModel,
}

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct UpdateUserInfo {
    pub id: String,
    pub user_info: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendAccountModel {
    pub email: String,
    pub password: String,
    pub role: String,
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

/////////////////////////////////// Category model

#[derive(Debug, SimpleObject, Serialize, Deserialize)]
pub struct CategoryResponseModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub image: File,
}

#[derive(Debug, SimpleObject, Clone, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub src: String,
}

/////////////////////////////////// Feature model

#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct FeatureResponseModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub feature_type: String,
    pub image: File,
    pub wireframes: Option<Vec<FileWithOutOId>>,
    pub price: f64,
    pub repo: String,
}
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct FileWithOutOId {
    pub id: String,
    pub name: String,
    pub src: String,
}

/////////////////////////////////// Template model

#[derive(Debug, Serialize, SimpleObject, Clone, Deserialize)]
pub struct TemplateResponseModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub features: Option<Vec<FeatureResponseModel>>,
    pub image: File,
    pub specification: Option<Specification>,
    // pub prototype_id: String,
}
#[derive(Debug, Serialize, SimpleObject, Deserialize)]
pub struct TemplateModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub features: Option<Vec<String>>,
    pub image: File,
    pub specification: Option<Specification>,
    // pub prototype_id: String,
}

#[derive(Debug, Clone, Serialize, SimpleObject, Deserialize)]
pub struct Specification {
    pub introduction: Introduction,
    pub overall_description: OverallDescription,
    pub non_functional_requirements: NonFunctionalRequirements,
    pub other_requirements: String,
    pub glossary: String,
    pub analysis_models: String,
    pub issues_list: String,
}

#[derive(Debug, Clone, Serialize, SimpleObject, Deserialize)]
pub struct Introduction {
    pub purpose: String,
    pub document_conventions: String,
    pub intended_audience: String,
    pub project_scope: String,
}

#[derive(Debug, Clone, Serialize, SimpleObject, Deserialize)]
pub struct OverallDescription {
    pub perspective: String,
    pub user_characteristics: String,
    pub operating_environment: String,
    pub design_implementation_constraints: String,
    pub user_documentation: String,
    pub assemptions_dependencies: String,
}

#[derive(Debug, Clone, Serialize, SimpleObject, Deserialize)]
pub struct NonFunctionalRequirements {
    pub performance_requirements: String,
    pub safety_requirements: String,
    pub security_requirements: String,
    pub software_quality_attributes: String,
}

#[derive(Debug, Serialize, SimpleObject, Deserialize)]
pub struct FeatureToAnyModel {
    pub id: String,
    pub features_id: Vec<String>,
}

#[derive(Debug, Clone, Serialize, InputObject, Deserialize)]
pub struct SpecificationInput {
    pub introduction: IntroductionInput,
    pub overall_description: OverallDescriptionInput,
    pub non_functional_requirements: NonFunctionalRequirementsInput,
    pub other_requirements: String,
    pub glossary: String,
    pub analysis_models: String,
    pub issues_list: String,
}

#[derive(Debug, Clone, Serialize, InputObject, Deserialize)]
pub struct IntroductionInput {
    pub purpose: String,
    pub document_conventions: String,
    pub intended_audience: String,
    pub project_scope: String,
}

#[derive(Debug, Clone, Serialize, InputObject, Deserialize)]
pub struct OverallDescriptionInput {
    pub perspective: String,
    pub user_characteristics: String,
    pub operating_environment: String,
    pub design_implementation_constraints: String,
    pub user_documentation: String,
    pub assemptions_dependencies: String,
}

#[derive(Debug, Clone, Serialize, InputObject, Deserialize)]
pub struct NonFunctionalRequirementsInput {
    pub performance_requirements: String,
    pub safety_requirements: String,
    pub security_requirements: String,
    pub software_quality_attributes: String,
}
