use std::{sync::Arc, time::Duration};

use dbus::blocking::Connection;
use kstars_dbus::ekos::capture::OrgKdeKstarsEkosCapture;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status, transport::Server};

use crate::ekos::{
    Empty, StartResponse, TrainRequest, TrainResponse,
    capture_server::{Capture, CaptureServer},
};

mod ekos {
    tonic::include_proto!("ekos");
}

pub struct EkosHandler {
    dbus: Arc<Mutex<Connection>>,
}

impl EkosHandler {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            dbus: Arc::new(Mutex::new(Connection::new_session()?)),
        })
    }
}

#[tonic::async_trait]
impl Capture for EkosHandler {
    async fn start(
        &self,
        request: Request<TrainRequest>,
    ) -> Result<Response<StartResponse>, Status> {
        let conn = self.dbus.lock().await;
        let proxy = conn.with_proxy(
            "org.kde.kstars",
            "/KStars/Ekos/Capture",
            Duration::from_millis(5000),
        );
        let res = OrgKdeKstarsEkosCapture::start(&proxy, &request.into_inner().train)
            .map_err(|e| Status::unknown(e.to_string()))?;
        Ok(Response::new(StartResponse { response: res }))
    }

    async fn abort(&self, request: Request<TrainRequest>) -> Result<Response<Empty>, Status> {
        let conn = self.dbus.lock().await;
        let proxy = conn.with_proxy(
            "org.kde.kstars",
            "/KStars/Ekos/Capture",
            Duration::from_millis(5000),
        );
        OrgKdeKstarsEkosCapture::abort(&proxy, &request.into_inner().train)
            .map_err(|e| Status::unknown(e.to_string()))?;
        Ok(Response::new(Empty {}))
    }

    async fn suspend(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        let conn = self.dbus.lock().await;
        let proxy = conn.with_proxy(
            "org.kde.kstars",
            "/KStars/Ekos/Capture",
            Duration::from_millis(5000),
        );
        OrgKdeKstarsEkosCapture::suspend(&proxy).map_err(|e| Status::unknown(e.to_string()))?;
        Ok(Response::new(Empty {}))
    }

    async fn pause(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        let conn = self.dbus.lock().await;
        let proxy = conn.with_proxy(
            "org.kde.kstars",
            "/KStars/Ekos/Capture",
            Duration::from_millis(5000),
        );
        OrgKdeKstarsEkosCapture::pause(&proxy).map_err(|e| Status::unknown(e.to_string()))?;
        Ok(Response::new(Empty {}))
    }

    async fn stop(&self, _request: Request<Empty>) -> Result<Response<Empty>, Status> {
        let conn = self.dbus.lock().await;
        let proxy = conn.with_proxy(
            "org.kde.kstars",
            "/KStars/Ekos/Capture",
            Duration::from_millis(5000),
        );
        OrgKdeKstarsEkosCapture::stop(&proxy).map_err(|e| Status::unknown(e.to_string()))?;
        Ok(Response::new(Empty {}))
    }

    async fn optical_train(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<TrainResponse>, Status> {
        let conn = self.dbus.lock().await;
        let proxy = conn.with_proxy(
            "org.kde.kstars",
            "/KStars/Ekos/Capture",
            Duration::from_millis(5000),
        );
        let res = OrgKdeKstarsEkosCapture::optical_train(&proxy)
            .map_err(|e| Status::unknown(e.to_string()))?;
        Ok(Response::new(TrainResponse { train: res }))
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;
    let ekos_handler = EkosHandler::new()?;

    Server::builder()
        .add_service(CaptureServer::new(ekos_handler))
        .serve(addr)
        .await?;

    Ok(())
}
