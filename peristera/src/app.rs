use fly_machines_gen::types::CreateAppRequest;

use crate::errors::Result;
use crate::{client::FlyMachinesClient, volume::FlyVolume};

pub struct FlyApp {
    pub(crate) client: FlyMachinesClient,
    pub(crate) app_name: String,
}

pub struct FlyAppBuilder {
    client: FlyMachinesClient,
    app_name: String,
    org: Option<String>,
    enable_subdomains: Option<bool>,
}

impl FlyAppBuilder {
    pub fn with_org(mut self, org: &str) -> Self {
        self.org = Some(org.to_string());
        self
    }

    pub fn with_enable_subdomains(mut self, enable_subdomains: bool) -> Self {
        self.enable_subdomains = Some(enable_subdomains);
        self
    }

    /// Given the parameters set in the builder, get an existing Fly Machine
    /// that matches these.
    pub async fn get(self) -> Result<FlyApp> {
        let resp = self
            .client
            .0
            .apps_show()
            .app_name(self.app_name)
            .send()
            .await;
        let unerrored_response = resp.unwrap();
        let app = unerrored_response.into_inner();
        Ok(FlyApp {
            client: self.client.clone(),
            app_name: app.name.unwrap(),
        })
    }

    pub async fn create(self) -> Result<FlyApp> {
        let body = CreateAppRequest {
            app_name: Some(self.app_name.clone()),
            org_slug: Some(self.org.unwrap_or("personal".to_string())),
            enable_subdomains: self.enable_subdomains,
            network: None,
        };
        let resp = self.client.0.apps_create().body(body).send().await;
        Ok(FlyApp {
            client: self.client.clone(),
            app_name: self.app_name,
        })
    }
}

impl FlyApp {
    pub async fn volume(&self, volume_id: &str) -> Result<FlyVolume> {
        let response = self
            .client
            .0
            .volumes_get_by_id()
            .app_name(&self.app_name)
            .volume_id(volume_id)
            .send()
            .await;
        unimplemented!()
    }
}
