/*
 * Chaos
 *
 * Central Management API - publicly exposed set of APIs for managing deployments
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`organisations_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganisationsCreateError {
    Status400(crate::models::BadRequestResponse),
    Status403(crate::models::UnauthorisedResponse),
    Status409(crate::models::ConflictResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organisations_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganisationsDeleteError {
    Status404(crate::models::NotFoundResponse),
    Status403(crate::models::UnauthorisedResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organisations_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganisationsGetError {
    Status404(crate::models::NotFoundResponse),
    Status403(crate::models::UnauthorisedResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organisations_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganisationsListError {
    Status403(crate::models::UnauthorisedResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`organisations_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrganisationsUpdateError {
    Status404(crate::models::NotFoundResponse),
    Status400(crate::models::BadRequestResponse),
    Status403(crate::models::UnauthorisedResponse),
    UnknownValue(serde_json::Value),
}


/// Create an organisation
pub async fn organisations_create(configuration: &configuration::Configuration, organisation_body: Option<crate::models::OrganisationBody>) -> Result<crate::models::OrganisationResponse, Error<OrganisationsCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organisations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&organisation_body);

    let local_var_req = local_var_req_builder.build()?;
    // FIXME: Remove after template fix
    // dbg!(&local_var_req);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OrganisationsCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete organisation, this will also delete all the resources within the organisation
pub async fn organisations_delete(configuration: &configuration::Configuration, organisation_id: &str) -> Result<crate::models::DeletedResponse, Error<OrganisationsDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organisations/{organisation_id}", local_var_configuration.base_path, organisation_id=crate::apis::urlencode(organisation_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    // FIXME: Remove after template fix
    // dbg!(&local_var_req);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OrganisationsDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a single organisation
pub async fn organisations_get(configuration: &configuration::Configuration, organisation_id: &str) -> Result<crate::models::OrganisationResponse, Error<OrganisationsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organisations/{organisation_id}", local_var_configuration.base_path, organisation_id=crate::apis::urlencode(organisation_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    // FIXME: Remove after template fix
    // dbg!(&local_var_req);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OrganisationsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List organisations
pub async fn organisations_list(configuration: &configuration::Configuration, page: Option<crate::models::OrganisationsListPageParameter>) -> Result<crate::models::ListOrganisationResponse, Error<OrganisationsListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organisations", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        #[derive(Serialize)]
        struct LocalStructDeepOrganisationsListPageParameter<'a> {
            r#page: &'a crate::models::OrganisationsListPageParameter,
        }
        let local_deep_struct = LocalStructDeepOrganisationsListPageParameter{ r#page: local_var_str };
        // let params = crate::apis::parse_deep_object("page", local_var_str);
        let params = crate::query_to_pairs(&local_deep_struct);
        local_var_req_builder = local_var_req_builder.query(&params);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    // FIXME: Remove after template fix
    // dbg!(&local_var_req);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OrganisationsListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update an organisation
pub async fn organisations_update(configuration: &configuration::Configuration, organisation_id: &str, organisation_body: Option<crate::models::OrganisationBody>) -> Result<crate::models::OrganisationResponse, Error<OrganisationsUpdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/organisations/{organisation_id}", local_var_configuration.base_path, organisation_id=crate::apis::urlencode(organisation_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&organisation_body);

    let local_var_req = local_var_req_builder.build()?;
    // FIXME: Remove after template fix
    // dbg!(&local_var_req);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OrganisationsUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

