use std::{collections::HashMap, error::Error, fs, sync::mpsc::Sender, thread::JoinHandle};

use backend::savefile::{Savefile, SavefileChanged};
use bevy::{
    app::{First, Plugin},
    ecs::{
        event::EventReader,
        system::{Res, ResMut, Resource},
    },
};
use rouille::{Response, Server};
use tracing::{error, info, warn};

pub struct WebserverPlugin;
impl Plugin for WebserverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<ServerResource>()
            .add_systems(First, savefile_changed);
    }
}

#[derive(Resource, Default)]
struct ServerResource {
    webserver: Option<Webserver>,
}

struct Webserver {
    signal: Sender<()>,
    handle: JoinHandle<()>,
}

fn savefile_changed(
    savefile: Res<Savefile>,
    mut server: ResMut<ServerResource>,
    mut savefile_changed_event: EventReader<SavefileChanged>,
) {
    if savefile_changed_event.is_empty() {
        return;
    }
    savefile_changed_event.clear();

    if let Some(webserver) = server.webserver.take() {
        info!("Stopping web server");
        _ = webserver.signal.send(());
        if webserver.handle.join().is_err() {
            error!("Webserver thread paniced");
        }
    }
    info!("Starting webserver");
    server.webserver = match start_webserver(&*savefile) {
        Ok((handle, signal)) => Some(Webserver { signal, handle }),
        Err(e) => {
            error!("Cannot start server: {e}");
            None
        }
    };
}

fn start_webserver(
    savefile: &Savefile,
) -> Result<(JoinHandle<()>, Sender<()>), Box<dyn Error + Sync + Send>> {
    // Load all assets for this savefile.
    let mut assets = HashMap::new();
    for asset in savefile.style().assets.contained_assets().into_iter() {
        let url = super::websocket::asset_to_uuid_asset_path(asset);
        let path = savefile.working_directory_path().join(&asset.path);
        match fs::read(&path) {
            Ok(data) => _ = assets.insert(url, data),
            Err(e) => warn!("Cannot read asset for webserver: {path:?}, {e}"),
        }
    }

    let server = Server::new("0.0.0.0:8000", move |request| {
        println!("Requested: {}: {}", request.method(), request.url());
        if request.method() != "GET" {
            return Response::empty_404();
        }

        match request.url().as_str() {
            "/index.html" => Response::from_data("text/html", *include_bytes!("../web/index.html")),
            "/restart-audio-context.js" => Response::from_data(
                "text/javascript",
                *include_bytes!("../web/restart-audio-context.js"),
            ),
            "/renderer/renderer.js" => Response::from_data(
                "text/javascript",
                *include_bytes!("../web/renderer/renderer.js"),
            ),
            "/renderer/renderer_bg.wasm" => Response::from_data(
                "application/wasm",
                *include_bytes!("../web/renderer/renderer_bg.wasm"),
            ),
            asset_id if asset_id.starts_with("/assets/") => {
                let uuid_str = asset_id
                    .strip_prefix("/assets/")
                    .expect("String does not start with correct pattern but should");

                let Some(asset_data) = assets.get(uuid_str) else {
                    warn!("Request for unknown asset: {uuid_str}");
                    return Response::empty_404();
                };
                Response::from_data("application/octet-stream", asset_data.as_slice())
            }
            _ => Response::empty_404(),
        }
    })?;
    info!("Starting webserver at {}", server.server_addr());
    Ok(server.stoppable())
}
