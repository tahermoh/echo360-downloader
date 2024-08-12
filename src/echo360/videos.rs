use std::fs::File;

use chrono::DateTime;
use reqwest::blocking::Client;
use serde::Deserialize;

use super::{courses::Section, Result};

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub lesson: LessonData,
    pub medias: Vec<Media>,
    pub has_content: bool,
    #[serde(rename(deserialize = "startTimeUTC"))]
    pub start_time_utc: Option<String>,
    #[serde(rename(deserialize = "endTimeUTC"))]
    pub end_time_utc: Option<String>,
    #[serde(skip)]
    pub download: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LessonData {
    id: String,
    pub display_name: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    id: String,
    title: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupInfo {
    group_id: String,
    pub name: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideosResponse {
    // status: String,
    // message: String,
    data: Vec<VideoData>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum VideoData {
    SyllabusLessonType {
        lesson: Video,
    },
    SyllabusGroupType {
        #[serde(rename(deserialize = "groupInfo"))]
        group_info: GroupInfo,
        lessons: Vec<VideoData>,
    },
}

impl Video {
    pub fn get_videos(
        client: &Client,
        domain: impl Into<String>,
        section_id: &String,
    ) -> Result<Vec<VideoData>> {
        let VideosResponse { data, .. } = client
            .get(&format!(
                "{}/section/{}/syllabus",
                domain.into(),
                section_id
            ))
            .send()
            .unwrap()
            .json::<VideosResponse>()
            .unwrap();

        Ok(dbg!(data))
    }

    pub fn download_videos(
        client: &Client,
        domain: impl Into<String>,
        code: &str,
        videos: &Vec<&Video>,
        path: impl Into<String>,
        captions: bool,
    ) -> Result<()> {
        let domain: String = domain.into();
        let path: String = path.into();
        for video in videos {
            let start = &video.start_time_utc;
            let name = match start {
                Some(time) => {
                    let date = DateTime::parse_from_rfc3339(&time).unwrap();
                    format!("{}{}_{}", &path, code, date.format("%Y-%m-%d"))
                }
                None => format!("{}{}", &path, video.lesson.display_name),
            };

            let mut video_file = File::create(name.clone() + ".mp4")?;
            let id = &video.medias.get(0).unwrap().id;
            client
                .get(&format!("{}/media/download/{}/hd1.mp4", &domain, id))
                .send()?
                .copy_to(&mut video_file)?;

            if captions {
                let mut caption_file = File::create(name + ".vtt")?;
                client
                    .get(&format!(
                        "{}/api/ui/echoplayer/lessons/{}/medias/{}/transcript-file?format=vtt",
                        &domain, video.lesson.id, id
                    ))
                    .send()?
                    .copy_to(&mut caption_file)?;
            }
        }
        Ok(())
    }
}
