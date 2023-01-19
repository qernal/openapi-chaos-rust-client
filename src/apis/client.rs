use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    functions_api: Box<dyn crate::apis::FunctionsApi>,
    organisations_api: Box<dyn crate::apis::OrganisationsApi>,
    projects_api: Box<dyn crate::apis::ProjectsApi>,
    providers_api: Box<dyn crate::apis::ProvidersApi>,
    system_api: Box<dyn crate::apis::SystemApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::connect::Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Rc::new(configuration);

        APIClient {
            functions_api: Box::new(crate::apis::FunctionsApiClient::new(rc.clone())),
            organisations_api: Box::new(crate::apis::OrganisationsApiClient::new(rc.clone())),
            projects_api: Box::new(crate::apis::ProjectsApiClient::new(rc.clone())),
            providers_api: Box::new(crate::apis::ProvidersApiClient::new(rc.clone())),
            system_api: Box::new(crate::apis::SystemApiClient::new(rc.clone())),
        }
    }

    pub fn functions_api(&self) -> &dyn crate::apis::FunctionsApi{
        self.functions_api.as_ref()
    }

    pub fn organisations_api(&self) -> &dyn crate::apis::OrganisationsApi{
        self.organisations_api.as_ref()
    }

    pub fn projects_api(&self) -> &dyn crate::apis::ProjectsApi{
        self.projects_api.as_ref()
    }

    pub fn providers_api(&self) -> &dyn crate::apis::ProvidersApi{
        self.providers_api.as_ref()
    }

    pub fn system_api(&self) -> &dyn crate::apis::SystemApi{
        self.system_api.as_ref()
    }

}
