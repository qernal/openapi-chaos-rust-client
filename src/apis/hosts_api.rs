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

use crate::models;
use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`projects_hosts_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectsHostsCreateError {
    Status404(models::NotFoundResponse),
    Status400(models::BadRequestResponse),
    Status409(models::ConflictResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`projects_hosts_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectsHostsDeleteError {
    Status404(models::NotFoundResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`projects_hosts_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectsHostsGetError {
    Status404(models::NotFoundResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`projects_hosts_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectsHostsListError {
    Status404(models::NotFoundResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`projects_hosts_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectsHostsUpdateError {
    Status404(models::NotFoundResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`projects_hosts_verify_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectsHostsVerifyCreateError {
    Status404(models::NotFoundResponse),
    Status400(models::BadRequestResponse),
    UnknownValue(serde_json::Value),
}


/// Assign a host/domain to a project - hosts are globally unique and require verification, so a host cannot be assigned to multiple projects.  A host can be a valid domain, either a root domain or a subdomain. 
pub async fn projects_hosts_create(configuration: &configuration::Configuration, project_id: &str, host_body: crate::models::HostBody) -> Result<models::Host, Error<ProjectsHostsCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_id}/hosts", local_var_configuration.base_path, project_id=crate::apis::urlencode(project_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&host_body);

    let local_var_req = local_var_req_builder.build()?;
    // FIXME: Remove after template fix
    // dbg!(&local_var_req);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ProjectsHostsCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn projects_hosts_delete(configuration: &configuration::Configuration, project_id: &str, hostname: &str) -> Result<models::DeletedResponse, Error<ProjectsHostsDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_id}/hosts/{hostname}", local_var_configuration.base_path, project_id=crate::apis::urlencode(project_id), hostname=crate::apis::urlencode(hostname));
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
        let local_var_entity: Option<ProjectsHostsDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn projects_hosts_get(configuration: &configuration::Configuration, project_id: &str, hostname: &str) -> Result<models::Host, Error<ProjectsHostsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_id}/hosts/{hostname}", local_var_configuration.base_path, project_id=crate::apis::urlencode(project_id), hostname=crate::apis::urlencode(hostname));
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
        let local_var_entity: Option<ProjectsHostsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn projects_hosts_list(configuration: &configuration::Configuration, project_id: &str, page: Option<crate::models::OrganisationsListPageParameter>) -> Result<models::ListHosts, Error<ProjectsHostsListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_id}/hosts", local_var_configuration.base_path, project_id=crate::apis::urlencode(project_id));
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
        let local_var_entity: Option<ProjectsHostsListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn projects_hosts_update(configuration: &configuration::Configuration, project_id: &str, hostname: &str, host_body_patch: crate::models::HostBodyPatch) -> Result<models::Host, Error<ProjectsHostsUpdateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_id}/hosts/{hostname}", local_var_configuration.base_path, project_id=crate::apis::urlencode(project_id), hostname=crate::apis::urlencode(hostname));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&host_body_patch);

    let local_var_req = local_var_req_builder.build()?;
    // FIXME: Remove after template fix
    // dbg!(&local_var_req);
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ProjectsHostsUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn projects_hosts_verify_create(configuration: &configuration::Configuration, project_id: &str, hostname: &str) -> Result<models::Host, Error<ProjectsHostsVerifyCreateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/projects/{project_id}/hosts/{hostname}/verify", local_var_configuration.base_path, project_id=crate::apis::urlencode(project_id), hostname=crate::apis::urlencode(hostname));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
        let local_var_entity: Option<ProjectsHostsVerifyCreateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
