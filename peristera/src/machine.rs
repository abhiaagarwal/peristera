use fly_machines_gen::types::{SignalRequest, SignalRequestSignal};

use crate::client::FlyMachinesClient;
use crate::errors::Result;


pub struct FlyMachine {
    client: FlyMachinesClient,
    app_name: String,
    machine_id: String,
}

pub struct FlyMachineBuilder {
    client: FlyMachinesClient,
}

impl FlyMachine {
    pub async fn signal(&self, signal: SignalRequestSignal) -> Result<()> {
        let body = SignalRequest {
            signal: Some(signal)
        };
        self.client.0.machines_signal().app_name(&self.app_name).body(body).send().await;
        unimplemented!()
    }
}