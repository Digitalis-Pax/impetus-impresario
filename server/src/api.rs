mod healthcheck;

use std::{net::IpAddr, path::PathBuf, thread};
use std::{
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::Router;
use axum_server::{Handle, tls_rustls::RustlsConfig};
use config::builder::BuilderState;
use log::{info, trace};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;

use crate::AppState;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ApiConfiguration {
    pub listen: Option<Vec<ListenConfiguration>>,
    pub tls: Option<TlsConfiguration>,
}

impl ApiConfiguration {
    pub fn new<DefaultState: BuilderState>(
        builder: config::ConfigBuilder<DefaultState>,
        _key_base: &str,
    ) -> config::ConfigBuilder<DefaultState> {
        builder
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ListenConfiguration {
    pub address: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TlsConfiguration {
    pub key: PathBuf,
    pub certificate: PathBuf,
}

pub struct Api {
    addresses: Vec<SocketAddr>,
    join_handles: Vec<JoinHandle<()>>,
    axum_handle: Handle,
    tls: Option<TlsConfiguration>,
}

impl Api {
    pub fn new(api_configuration: &ApiConfiguration) -> Self {
        let addresses = match &api_configuration.listen {
            Some(listens) => listens
                .iter()
                .map(|l| SocketAddr::new(IpAddr::from_str(&l.address).unwrap(), l.port))
                .collect(),
            None => vec![SocketAddr::new(IpAddr::from_str("::").unwrap(), 8443)],
        };

        Self {
            join_handles: vec![],
            axum_handle: axum_server::Handle::new(),
            addresses,
            tls: api_configuration.tls.clone(),
        }
    }

    pub async fn serve(self: &mut Self, app_state: &Arc<Mutex<AppState>>) {
        let mut router: Router<Arc<Mutex<AppState>>> = Router::new();
        router = healthcheck::add_routes(router);

        let tls = Self::build_tls(&self.tls).await;

        for address in &self.addresses {
            self.join_handles
                .push(self.start_server(app_state, &router, &tls, address));
        }
    }

    fn start_server(
        self: &Self,
        app_state: &Arc<Mutex<AppState>>,
        router: &Router<Arc<Mutex<AppState>>>,
        tls: &Option<RustlsConfig>,
        address: &SocketAddr,
    ) -> JoinHandle<()> {
        let server_state = app_state.clone();
        let server_routes = router.clone().with_state(server_state);
        let server_address = address.clone();
        let server_tls = tls.clone();
        let server_handle = self.axum_handle.clone();

        tokio::spawn(async move {
            match server_tls {
                Some(t) => {
                    trace!("Attempting to serve HTTPS on {:?}", server_address);
                    axum_server::bind_rustls(server_address, t)
                        .handle(server_handle)
                        .serve(server_routes.into_make_service())
                        .await
                        .unwrap();
                }
                None => {
                    trace!("Attempting to serve HTTP on {:?}", server_address);
                    axum_server::bind(server_address)
                        .handle(server_handle)
                        .serve(server_routes.into_make_service())
                        .await
                        .unwrap();
                }
            };
        })
    }

    async fn build_tls(tls: &Option<TlsConfiguration>) -> Option<RustlsConfig> {
        let tls: Option<RustlsConfig> = match tls {
            Some(t) => Some(
                RustlsConfig::from_pem_file(&t.certificate, &t.key)
                    .await
                    .unwrap(),
            ),
            None => None,
        };
        tls
    }

    pub fn shutdown(self: &mut Self) {
        self.axum_handle
            .graceful_shutdown(Some(Duration::from_millis(100)));
    }
}

impl Drop for Api {
    fn drop(&mut self) {
        self.shutdown();
        let mut shutdown_complete = false;
        while !shutdown_complete {
            let mut someone_is_alive = false;
            for handle in &self.join_handles {
                if !handle.is_finished() {
                    someone_is_alive = true;
                    break;
                }
            }
            shutdown_complete = !someone_is_alive;
            thread::sleep(Duration::from_millis(100));
        }
        info!("API server shutdown successfully.")
    }
}
