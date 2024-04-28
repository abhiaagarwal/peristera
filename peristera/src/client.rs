use crate::app::FlyApp;
use crate::errors::Result;
use fly_machines_gen::Client;
use reqwest::{
    header::{self, HeaderMap},
    ClientBuilder,
};
use url::Url;

/// A client that represents a connection to the Fly Machines API.
#[derive(Clone)]
pub struct FlyMachinesClient(pub(crate) Client);

impl FlyMachinesClient {
    /// Create a new Fly Machines Client, with a connection via the specified endpoint and token. If `None`,
    /// the endpoint is set to https://api.machines.dev.
    ///
    /// A token can be retrieved with `flyctl auth token`.
    ///
    /// This does not validate that the specified endpoint is valid, only that is it is a valid URL.
    /// It also does not validate that the token is working. Call any method to check that.
    pub fn new(endpoint: Option<&Url>, token: &str) -> Self {
        let builder = {
            let dur = std::time::Duration::from_secs(15);
            let mut headers = HeaderMap::new();
            headers.insert(
                header::AUTHORIZATION,
                format!("Bearer {token}").parse().unwrap(),
            );
            let builder = ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
                .default_headers(headers);
            builder
        };
        Self(Client::new_with_client(
            endpoint.map_or("https://api.machines.dev", |u| u.as_str()),
            builder.build().unwrap(),
        ))
    }

    /// List all apps associated with this client. If an "org" is not specified, it will refer to
    /// the "personal" org.
    pub async fn apps(&self, org: Option<&str>) -> Result<Vec<FlyApp>> {
        let res = self
            .0
            .apps_list()
            .org_slug(org.unwrap_or("personal"))
            .send()
            .await;
        let val = res.unwrap();
        let apps: Vec<FlyApp> = val
            .into_inner()
            .apps
            .into_iter()
            .map(|resp_app| FlyApp {
                client: self.clone(),
                app_name: resp_app.name.ok_or(Err("Bad!"))?,
            })
            .collect();
        Ok(apps)
    }
}
