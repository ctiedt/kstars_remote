use std::{pin::Pin, sync::Arc, time::Duration};

use dbus::{Message, arg::RefArg, blocking::Connection};
use kstars_dbus::ekos::capture::{OrgKdeKstarsEkosCapture, OrgKdeKstarsEkosCaptureCaptureComplete};
use tokio::sync::Mutex;
use tokio_stream::{
    Stream, StreamExt,
    wrappers::{BroadcastStream, errors::BroadcastStreamRecvError},
};
use tonic::{Request, Response, Status, transport::Server};

use crate::ekos::{
    CaptureComplete, Empty, StartResponse, TrainRequest, TrainResponse,
    capture_server::{Capture, CaptureServer},
};

mod ekos {
    tonic::include_proto!("ekos");
}

pub struct EkosHandler {
    dbus: Arc<Mutex<Connection>>,
    capture_signal_queue: tokio::sync::broadcast::Receiver<CaptureComplete>,
    capture_signal_tx: tokio::sync::broadcast::Sender<CaptureComplete>,
}

impl EkosHandler {
    pub fn new() -> color_eyre::Result<Self> {
        let conn = Connection::new_session()?;
        let (tx, rx) = tokio::sync::broadcast::channel(1024);
        Ok(Self {
            dbus: Arc::new(Mutex::new(conn)),
            capture_signal_queue: rx,
            capture_signal_tx: tx,
        })
    }

    pub fn handle_signals(&self) -> color_eyre::Result<()> {
        let conn = Connection::new_session()?;
        let capture_proxy = conn.with_proxy(
            "org.kde.kstars",
            "/KStars/Ekos/Capture",
            Duration::from_millis(5000),
        );
        let tx = self.capture_signal_tx.clone();
        capture_proxy.match_signal(
            move |sig: OrgKdeKstarsEkosCaptureCaptureComplete, _: &Connection, _: &Message| {
                dbg!(&sig.train, &sig.metadata);
                tx.send(CaptureComplete {
                    train: sig.train,
                    exposure: sig.metadata.get("exposure").unwrap().as_f64().unwrap(),
                    filter: sig
                        .metadata
                        .get("filter")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                    width: sig.metadata.get("width").unwrap().as_i64().unwrap(),
                    height: sig.metadata.get("height").unwrap().as_i64().unwrap(),
                    r#type: sig.metadata.get("type").unwrap().as_i64().unwrap() as i32,
                    filename: sig
                        .metadata
                        .get("filename")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                })
                .unwrap();
                true
            },
        )?;
        std::thread::spawn(move || -> color_eyre::Result<()> {
            loop {
                conn.process(Duration::from_millis(1000))?;
            }
        });
        Ok(())
    }
}

#[tonic::async_trait]
impl Capture for EkosHandler {
    type CaptureCompleteSignalStream =
        Pin<Box<dyn Stream<Item = Result<CaptureComplete, Status>> + Send>>;

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

    async fn capture_complete_signal(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::CaptureCompleteSignalStream>, Status> {
        let stream =
            BroadcastStream::new(self.capture_signal_queue.resubscribe()).map(|v| match v {
                Ok(v) => Ok(v),
                Err(e) => Err(Status::unknown(e.to_string())),
            });
        Ok(Response::new(
            Box::pin(stream) as Self::CaptureCompleteSignalStream
        ))
    }
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;
    let ekos_handler = EkosHandler::new()?;
    ekos_handler.handle_signals()?;

    Server::builder()
        .add_service(CaptureServer::new(ekos_handler))
        .serve(addr)
        .await?;

    Ok(())
}
