use std::process::{Child, Command};

use anyhow::Error;

use std::time::Duration;

use tokio::time::sleep;

pub struct Llama {
    pub server: Option<Child>,
    pub executable: String,
    pub model: String,
    pub port: u16,
    pub context: usize,
    pub gpu_layers: usize,
}

impl Llama {
    pub fn new() -> Self {
        Self {
            server: None,
            executable: "C:\\llama\\llama-server.exe".to_string(),
            model: "C:\\llama\\models\\qwen2.5-coder-7b-instruct-q4_k_m.gguf".to_string(),
            port: 8080,
            context: 4096,
            gpu_layers: 15,
        }
    }

    pub fn start(&mut self) -> Result<(), std::io::Error> {
        let server = Command::new(&self.executable)
            .arg("-m")
            .arg(&self.model)
            .arg("-ngl")
            .arg(self.gpu_layers.to_string())
            .arg("-c")
            .arg(self.context.to_string())
            .arg("--host")
            .arg("127.0.0.1")
            .arg("--port")
            .arg(self.port.to_string())
            .spawn();
        
        self.server = Some(server?);

        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), Error> {
        if let Some(mut server) = self.server.take() {
            server.kill()?;
            server.wait()?;
        }

        Ok(())
    }
    pub async fn check_health(&self) -> Result<bool, reqwest::Error> {
        let response = reqwest::get(format!("http://127.0.0.1:{}/health", self.port))
            .await?;

        Ok(response.status().is_success())
    }

    pub async fn wait_until_ready(&self) -> Result<(), Error> {
        println!("[Waiting for llama-server...]");

        loop {
            if self.check_health().await.unwrap_or(false) {
                println!("[llama-server ready]");
                break;
            }

            sleep(Duration::from_millis(500)).await;
        }

        Ok(())
    }
}
