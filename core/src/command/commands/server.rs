use anyhow::{Error, Ok};

use log::{info, warn};

use crate::Llama;

pub async fn ao_start_server(llama: &mut Llama) -> Result<(), Error> {
    llama.start()?;
    llama.wait_until_ready().await?;
    info!("[llama-server started]");

    Ok(())
}

pub fn ao_stop_server(llama: &mut Llama) -> Result<(), Error> {
    if llama.server.is_some() {
        llama.stop()?;

        info!("[llama-server stopped]");
    } else {
        warn!("[llama-server is not running]");
    }

    Ok(())
}

pub fn ao_set_context(llama: &mut Llama, context: usize) {
    llama.context = context;
    info!("[llama-server context: {}]", llama.context);
}
pub fn ao_set_gpu_layers(llama: &mut Llama, layers: usize) {
    llama.gpu_layers = layers;
    info!("[llama-server gpu_layers: {}]", llama.gpu_layers);
}
