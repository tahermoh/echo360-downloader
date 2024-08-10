mod error;
// use error::Result;

use crate::{
    echo360::{courses::Enrollments, Echo360},
    task::{Task, TaskState},
};
use eframe::egui::{self, Context, RichText, Ui};

#[derive(Default)]
pub struct App {
    echo360: Task<Echo360>,
    state: AppState,
}

#[derive(Default)]
enum AppState {
    #[default]
    LoggingIn,
    LoadingCourses,
    SelectingCourse,
    LoadingVideos,
    SelectingVideos,
}

impl App {
    fn login_screen(&mut self, ctx: &Context) {
        let login_button = |echo360: &mut Task<Echo360>, ui: &mut Ui| {
            if ui
                .add(egui::Button::new(
                    RichText::new("Log in to see Courses")
                        .size(32.0)
                        .heading()
                        .strong(),
                ))
                .clicked()
            {
                echo360.fire_async(async move { Echo360::login().await.unwrap() });
            };
        };

        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            ui.add_space(10.0);

            ui.vertical_centered(|ui| match self.echo360.state() {
                TaskState::NotFired => {
                    login_button(&mut self.echo360, ui);
                }
                TaskState::Loading(_) => {
                    ui.add(egui::Spinner::new().size(39.0));
                }
                TaskState::Ok(_) => {
                    self.state = AppState::LoadingCourses;
                }
            });

            ui.add_space(10.0);
        });
    }

    fn course_select_screen(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Choose a Course")
                        .size(32.0)
                        .heading()
                        .strong(),
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("enrollments").show(ui, |ui| {
                if let Some(enrollments) = &self.echo360.get().unwrap().enrollments.get() {
                    for (i, enrollment) in enrollments.user_sections.iter().enumerate() {
                        if ui
                            .add(
                                egui::Button::new(&enrollment.section_name)
                                    .min_size([200., 50.].into()),
                            )
                            .clicked()
                        {
                            // Select class
                        }

                        if i % 3 == 2 {
                            ui.end_row()
                        }
                    }
                }
            });
        });
    }

    fn load_courses(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("Loading Courses")
                        .size(32.0)
                        .heading()
                        .strong(),
                );
            });
        });

        let echo360 = self.echo360.get().unwrap();

        echo360
            .enrollments
            .set(Enrollments::get(&echo360.client, &echo360.domain).unwrap())
            .unwrap();
        self.state = AppState::SelectingCourse;
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        match self.state {
            AppState::LoggingIn => self.login_screen(ctx),
            AppState::LoadingCourses => self.load_courses(ctx),
            AppState::SelectingCourse => self.course_select_screen(ctx),
            AppState::LoadingVideos => todo!(),
            AppState::SelectingVideos => todo!(),
        };
    }
}
