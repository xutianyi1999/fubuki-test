use std::pin::pin;

use anyhow::Result;
use futures_util::stream::StreamExt;
use net_route::{Handle, Route};

pub struct SystemRouteHandle {
    handle: Handle,
    routes: Vec<Route>,
    rt: tokio::runtime::Handle,
}

impl SystemRouteHandle {
    pub fn new() -> Result<Self> {
        let handle = Handle::new()?;
        let stream = handle.route_listen_stream();

        tokio::spawn(async move {
            let mut stream = pin!(stream);

            while let Some(v) = stream.next().await {
                debug!("route change: {:?}", v)
            }
        });

        let this = SystemRouteHandle {
            handle,
            routes: Vec::new(),
            rt: tokio::runtime::Handle::current(),
        };
        Ok(this)
    }

    pub async fn add(&mut self, routes: &[Route]) -> Result<()> {
        for x in routes {
            #[cfg(target_os = "macos")]
            {
                use std::process::Stdio;
                use anyhow::anyhow;
                use tokio::process::Command;

                let status = Command::new("route")
                    .args([
                        "-n",
                        "add",
                        "-net",
                        x.destination.to_string().as_str(),
                        "-netmask",
                        x.mask().to_string().as_str(),
                        x.gateway.unwrap().to_string().as_str(),
                    ])
                    .stderr(Stdio::inherit())
                    .output()
                    .await?
                    .status;

                if !status.success() {
                    return Err(anyhow!("failed to add route"));
                }
            }

            #[cfg(not(target_os = "macos"))]
            self.handle.add(x).await?;

            self.routes.push(x.clone());
        }
        Ok(())
    }

    pub async fn clear(&mut self) -> Result<()> {
        for x in &self.routes {
            self.handle.delete(x).await?;
        }

        self.routes = Vec::new();
        Ok(())
    }
}

impl Drop for SystemRouteHandle {
    fn drop(&mut self) {
        if !self.routes.is_empty() {
            info!("clear all routes");

            let rt= self.rt.clone();

            std::thread::scope(|scope| {
                scope.spawn(|| {
                    if let Err(e) = rt.block_on(self.clear()) {
                        warn!("delete route failure: {}", e)
                    }
                });
            });
        }
    }
}