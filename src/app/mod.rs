mod error;
// use error::Result;

use std::{cell::OnceCell, ops::DerefMut};

use crate::echo360::{
    courses::Enrollments,
    videos::{Video, VideoData},
    Echo360,
};
use chrono::{DateTime, Local};
use eframe::egui::{self, Context, RichText, Ui};
use egui_extras::{Column, TableRow};

#[derive(Default)]
pub struct App {
    echo360: OnceCell<Echo360>,
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
        let mut login_button = |echo360: &mut OnceCell<Echo360>, ui: &mut Ui| {
            if ui
                .add(egui::Button::new(
                    RichText::new("Log in to see Courses")
                        .size(32.0)
                        .heading()
                        .strong(),
                ))
                .clicked()
            {
                let _ = echo360.set(Echo360::login().unwrap());
                ui.add(egui::Spinner::new().size(39.0));
                self.state = AppState::LoadingCourses;
            };
        };

        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                login_button(&mut self.echo360, ui);
                ui.add_space(10.0);
            });
        });
    }

    fn course_select_screen(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(
                    RichText::new("Choose a Course")
                        .size(32.0)
                        .heading()
                        .strong(),
                );
                ui.add_space(10.0);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let echo360 = self.echo360.get().unwrap();

            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.columns(3, |columns| {
                        if let Some(enrollments) = echo360.enrollments.get() {
                            for (i, enrollment) in enrollments.user_sections.iter().enumerate() {
                                columns[i % 3].vertical_centered(|ui| {
                                    if ui
                                        .add(
                                            egui::Button::new(&enrollment.section_name)
                                                .min_size([200., 50.].into()),
                                        )
                                        .clicked()
                                    {
                                        // Select class
                                        echo360.selected.replace(enrollment.section_id.clone());
                                        self.state = AppState::LoadingVideos;
                                    }
                                });
                            }
                        }
                    });
                });
        });
    }

    fn load_courses(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(
                    RichText::new("Loading Courses")
                        .size(32.0)
                        .heading()
                        .strong(),
                );
                ui.add_space(10.0);
            });
        });

        let echo360 = self.echo360.get().unwrap();

        echo360
            .enrollments
            .set(Enrollments::get(&echo360.client, &echo360.domain).unwrap())
            .unwrap();
        self.state = AppState::SelectingCourse;
    }

    fn load_videos(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.label(
                    RichText::new("Loading Videos")
                        .size(32.0)
                        .heading()
                        .strong(),
                );
                ui.add_space(10.0);
            });
        });

        let echo360 = self.echo360.get().unwrap();

        echo360.videos.replace(dbg!(Video::get_videos(
            &echo360.client,
            &echo360.domain,
            &echo360.selected.borrow()
        )
        .unwrap()));
        self.state = AppState::SelectingVideos;
    }

    fn video_select_screen(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if ui.button("Back").clicked() {
                    self.state = AppState::SelectingCourse;
                    return;
                };
                ui.centered_and_justified(|ui| {
                    ui.label(RichText::new("Choose Videos").size(32.0).heading().strong());
                });
            });
        });
        egui::TopBottomPanel::bottom("Bottom Panel")
            .min_height(50.)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.label("Download path:");
                    ui.text_edit_singleline(
                        self.echo360
                            .get()
                            .unwrap()
                            .download_path
                            .borrow_mut()
                            .deref_mut(),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(20.);
                        if ui.button("Download Videos").clicked() {};
                        ui.add_space(20.);
                        ui.add(toggle(&mut true));
                        ui.label("Download Captions");
                    });
                });
            });

        let echo360 = self.echo360.get().unwrap();
        let mut videos = echo360.videos.borrow_mut();

        let entry = |mut row: TableRow, lesson: &mut Video| {
            let show = lesson.has_content.clone();
            let name = lesson.lesson.display_name.clone();
            let start = lesson.start_time_utc.as_ref();
            let end = lesson.end_time_utc.as_ref();

            row.col(|ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_enabled_ui(show, |ui| {
                        ui.label(name);
                    });
                });
            });
            row.col(|ui| {
                match (
                    DateTime::parse_from_rfc3339(start.unwrap_or(&"".to_string())),
                    DateTime::parse_from_rfc3339(end.unwrap_or(&"".to_string())),
                ) {
                    (Ok(start), Ok(end)) => {
                        let start_time: DateTime<Local> = DateTime::from(start);
                        let end_time: DateTime<Local> = DateTime::from(end);

                        ui.label(format!(
                            "{}-{}",
                            start_time.format("%d/%m/%Y %H:%M"),
                            end_time.format("%H:%M")
                        ));
                    }
                    _ => {
                        ui.label("---");
                    }
                };
            });
            row.col(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.add_enabled(show, toggle(&mut lesson.download));
                });
            });
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            egui_extras::TableBuilder::new(ui)
                .column(Column::remainder())
                .column(Column::auto())
                .column(Column::auto())
                .striped(true)
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Recording Name");
                    });
                    header.col(|ui| {
                        ui.heading("Time");
                    });
                    header.col(|ui| {
                        ui.heading("Download");
                    });
                })
                .body(|mut body| {
                    for video_data in videos.iter_mut() {
                        match video_data {
                            VideoData::SyllabusLessonType { lesson } => {
                                body.row(30.0, |row| {
                                    entry(row, lesson);
                                });
                            }
                            VideoData::SyllabusGroupType {
                                group_info,
                                lessons,
                            } => {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.horizontal_centered(|ui| {
                                            ui.heading(&group_info.name);
                                        });
                                    });
                                });
                                for video_data in lessons {
                                    match video_data {
                                        VideoData::SyllabusLessonType { lesson } => {
                                            body.row(30.0, |row| {
                                                entry(row, lesson);
                                            });
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                });
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        match self.state {
            AppState::LoggingIn => self.login_screen(ctx),
            AppState::LoadingCourses => self.load_courses(ctx),
            AppState::SelectingCourse => self.course_select_screen(ctx),
            AppState::LoadingVideos => self.load_videos(ctx),
            AppState::SelectingVideos => self.video_select_screen(ctx),
        };
    }
}

fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(toggle(&mut my_bool))`
/// iOS-style toggle switch.
///
/// ## Example:
/// ``` ignore
/// ui.add(toggle(&mut my_bool));
/// ```
pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}
