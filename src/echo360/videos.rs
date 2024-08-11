use reqwest::Client;
use serde::Deserialize;

use super::Result;

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
    pub async fn get_videos(
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
            .await
            .unwrap()
            .json::<VideosResponse>()
            .await
            .unwrap();

        Ok(dbg!(data))
    }
}
