#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unsafe_code)]

use anyhow::{anyhow, Context, Result};

mod app;

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info).context("Error Initialising the logger")?;
    let options = eframe::NativeOptions {
        multisampling: 8,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Custom 3D painting in eframe using glow",
        options,
        Box::new(|cc| Ok(Box::new(app::FractalApp::new(cc)?))),
    )
    .map_err(|e| anyhow!("Error when starting the eframe Framework: {:?}", e))
}
