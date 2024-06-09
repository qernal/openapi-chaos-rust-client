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


/// struct for typed errors of method [`providers_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProvidersListError {
    UnknownValue(serde_json::Value),
}


/// Retrieve a list of all providers with their respective deployed regions and cities.
pub async fn providers_list(configuration: &configuration::Configuration, page: Option<crate::models::OrganisationsListPageParameter>) -> Result<models::ListProviderResponse, Error<ProvidersListError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/providers", local_var_configuration.base_path);
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
        let local_var_entity: Option<ProvidersListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

