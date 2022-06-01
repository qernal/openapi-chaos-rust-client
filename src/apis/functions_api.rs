/*
 * Chaos
 *
 * Management API - Central user exposed system
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@qernal.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`delete_projects_project_id_functions_function_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteProjectsProjectIdFunctionsFunctionIdError {
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_projects_project_id_functions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProjectsProjectIdFunctionsError {
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_projects_project_id_functions_function_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProjectsProjectIdFunctionsFunctionIdError {
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_projects_project_id_functions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostProjectsProjectIdFunctionsError {
    Status400(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_projects_project_id_functions_function_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutProjectsProjectIdFunctionsFunctionIdError {
    Status400(serde_json::Value),
    Status404(serde_json::Value),
    UnknownValue(serde_json::Value),
}


/// Delete specific function
pub async fn delete_projects_project_id_functions_function_id(configuration: &configuration::Configuration, project_id: &str, function_id: &str) -> Result<serde_json::Value, Error<DeleteProjectsProjectIdFunctionsFunctionIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project-id}/functions/{function-id}", local_var_configuration.base_path, project-id=crate::apis::urlencode(project_id), function-id=crate::apis::urlencode(function_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteProjectsProjectIdFunctionsFunctionIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the functions of a project
pub async fn get_projects_project_id_functions(configuration: &configuration::Configuration, project_id: &str) -> Result<Vec<serde_json::Value>, Error<GetProjectsProjectIdFunctionsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project-id}/functions", local_var_configuration.base_path, project-id=crate::apis::urlencode(project_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetProjectsProjectIdFunctionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a specific function
pub async fn get_projects_project_id_functions_function_id(configuration: &configuration::Configuration, project_id: &str, function_id: &str) -> Result<serde_json::Value, Error<GetProjectsProjectIdFunctionsFunctionIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project-id}/functions/{function-id}", local_var_configuration.base_path, project-id=crate::apis::urlencode(project_id), function-id=crate::apis::urlencode(function_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetProjectsProjectIdFunctionsFunctionIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a function on a project
pub async fn post_projects_project_id_functions(configuration: &configuration::Configuration, project_id: &str, create_project_functions_request_body_schema: Option<crate::models::CreateProjectFunctionsRequestBodySchema>) -> Result<serde_json::Value, Error<PostProjectsProjectIdFunctionsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project-id}/functions", local_var_configuration.base_path, project-id=crate::apis::urlencode(project_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&create_project_functions_request_body_schema);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostProjectsProjectIdFunctionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a specific function
pub async fn put_projects_project_id_functions_function_id(configuration: &configuration::Configuration, project_id: &str, function_id: &str, body: Option<serde_json::Value>) -> Result<serde_json::Value, Error<PutProjectsProjectIdFunctionsFunctionIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project-id}/functions/{function-id}", local_var_configuration.base_path, project-id=crate::apis::urlencode(project_id), function-id=crate::apis::urlencode(function_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutProjectsProjectIdFunctionsFunctionIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

