use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

const TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiJjbWdkbjdlcjkwMDAwb3lhZmwxeXVoZ2FyIiwiZW1haWwiOiJ0ZXN0QGdtYWlsLmNvbSIsInNlc3Npb25JZCI6ImNtZ2ZtYnhjMjAwMDFveTZvMmRiazg5c3AiLCJpYXQiOjE3NTk3ODQ2MjgsImV4cCI6MTc2MDM4OTQyOH0.96Cd8VbBZ8gjNpsfYH6P3GJpGUZaWHnCsZ2bYYlQCJ8";


#[derive(Clone, Deserialize, PartialEq, Debug)]
struct Course {
    #[serde(rename = "courseId")]
    course_id: String,
    #[serde(rename = "courseName")]
    course_name: String,
    thumbnail: String,
    description: String,
    language: String,
    #[serde(rename = "courseTitles")]
    course_titles: Vec<String>,
    #[serde(rename = "courseVideos")]
    course_videos: Vec<String>,
}

#[derive(Properties, PartialEq)]
pub struct CourseDetailProps {
    pub id: String,
}

#[function_component(CourseDetail)]
pub fn course_detail(props: &CourseDetailProps) -> Html {
    let course = use_state(|| None::<Course>);
    let selected_video = use_state(|| None::<String>);
    let course_id = props.id.clone();

    {
        let course = course.clone();
        use_effect_with(course_id.clone(), move |course_id_ref| {
            // Clone the string so the async block owns it
            let course_id = course_id_ref.clone();

            spawn_local(async move {
                let url = format!("http://localhost:5000/api/user/course/{}", course_id);

                match Request::get(&url)
                    .header("Authorization", TOKEN)
                    .send()
                    .await
                {
                    Ok(resp) => match resp.json::<serde_json::Value>().await {
                        Ok(json) => {
                            console::log_1(&format!("Response JSON: {:?}", json).into());
                            if let Ok(parsed) =
                                serde_json::from_value::<Course>(json["Course"].clone())
                            {
                                course.set(Some(parsed));
                            } else {
                                console::log_1(&"Failed to parse Course object".into());
                            }
                        }
                        Err(e) => console::log_1(&format!("JSON parse error: {:?}", e).into()),
                    },
                    Err(e) => console::log_1(&format!("Request failed: {:?}", e).into()),
                }
            });

            || ()
        });
    }

    html! {
        <div class="p-6">
            {
                if let Some(course) = (*course).clone() {
                    let current_video = (*selected_video)
                        .clone()
                        .unwrap_or_else(|| course.course_videos[0].clone());

                    let embed_video = if current_video.contains("youtu.be") || current_video.contains("youtube.com/watch") {
                        if current_video.contains("youtu.be") {
                            let id = current_video.split('/').last().unwrap_or("");
                            format!("https://www.youtube.com/embed/{}", id)
                        } else {
                            let query = current_video.split("v=").nth(1).unwrap_or("");
                            let id = query.split('&').next().unwrap_or("");
                            format!("https://www.youtube.com/embed/{}", id)
                        }
                    } else {
                        current_video.clone()
                    };

                    html! {
                        <div class="max-w-4xl mx-auto">
                            <div class="mb-6">
                                <img
                                    src={course.thumbnail.clone()}
                                    alt={course.course_name.clone()}
                                    class="rounded mb-4 w-full max-h-96 object-cover"
                                />
                                <h1 class="text-3xl font-bold mb-2">{ &course.course_name }</h1>
                                <p class="text-gray-600 mb-2">{ &course.description }</p>
                                <p class="text-sm text-gray-400 italic">
                                    { format!("Language: {}", &course.language) }
                                </p>
                            </div>

                            <div class="mb-6 aspect-video">
                                <iframe
                                    width="100%"
                                    height="480"
                                    src={embed_video}
                                    title="Course Video Player"
                                    class="rounded w-full"
                                    frameborder="0"
                                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                                    allowfullscreen=true
                                />
                            </div>

                            <div class="space-y-3">
                                { for course.course_titles.iter().zip(course.course_videos.iter()).enumerate().map(|(index, (title, video))| {
                                    let video_link = video.clone();
                                    let selected_video = selected_video.clone();

                                    html! {
                                        <div
                                            key={index}
                                            class="p-3 bg-gray-100 rounded cursor-pointer hover:bg-gray-200 transition"
                                            onclick={Callback::from(move |_| {
                                                selected_video.set(Some(video_link.clone()));
                                            })}
                                        >
                                            <p class="text-lg font-medium">{ format!("▶ {}", title) }</p>
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="text-center text-gray-500">
                            {"Loading course details..."}
                        </div>
                    }
                }
            }
        </div>
    }
}
