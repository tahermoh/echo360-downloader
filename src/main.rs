#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, RichText};
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

pub mod task;
use task::{Task, TaskState};

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

struct App {
    cookie: Task<Result<Cookie, Error>>,
    //   runtime: Runtime,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cookie: Task::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            ui.add_space(10.0);

            ui.vertical_centered(|ui| match self.cookie.state() {
                TaskState::NotFired => {
                    if ui
                        .add(egui::Button::new(
                            RichText::new("Log in to see Courses")
                                .size(32.0)
                                .heading()
                                .strong(),
                        ))
                        .clicked()
                    {
                        self.cookie.fire_async(async move { login().await });
                    };
                }
                TaskState::Loading(_) => {
                    ui.add(egui::Spinner::new().size(39.0));
                }
                TaskState::Ok(cookie) => {
                    // Check if log in finished
                    if cookie.is_ok() {
                        ui.label(egui::RichText::new("Logged In!").size(32.0));
                    } else {
                        if ui
                            .add(egui::Button::new(
                                RichText::new("Log in to see Courses")
                                    .size(32.0)
                                    .heading()
                                    .strong(),
                            ))
                            .clicked()
                        {
                            self.cookie.fire_async(async move { login().await });
                        };
                    }
                }
            });

            ui.add_space(10.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                if ui.label("test").clicked() {
                    println!("hm");
                };
            });
        });
    }
}

async fn login() -> Result<Cookie, Error> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://login.echo360.org/login").await?;

    loop {
        // Busy loop until logged in
        let url = driver.current_url().await?;
        if let Some(domain) = url.domain() {
            if domain.starts_with("echo360") {
                break;
            };
        }
        sleep(Duration::from_millis(1000)).await;
    }

    let cookie = driver.get_named_cookie("PLAY_SESSION").await?;

    driver.quit().await?;

    Ok(cookie)
}
