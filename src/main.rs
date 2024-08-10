#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use thirtyfour::prelude::*;

mod app;
mod echo360;
mod task;

use app::App;

#[derive(Debug)]
pub enum Error {
    LoginFail(WebDriverError),
}

impl From<WebDriverError> for Error {
    fn from(err: WebDriverError) -> Self {
        Self::LoginFail(err)
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().expect("Unable to create background threads");
    let _enter = rt.enter();

    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "Echo360 Video Downloader",
        options,
        Box::new(|cc| {
            // Image support
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(App::default())
        }),
    )
    .unwrap();
}
