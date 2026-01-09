use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

const TOKEN: &str = "";


#[derive(Clone, Deserialize, PartialEq, Debug)]
struct Course {
    #[serde(rename = "courseId")]
    course_id: String,
    #[serde(rename = "courseName")]
    course_name: String,
    thumbnail: String,
    description: String,
    language: String,
}

/// Enrolls the current user in a course.
async fn enroll(course_id: String) {
    let url = format!("http://localhost:5000/api/user/enroll/{}", course_id);
    let res = Request::post(&url)
        .header("Authorization", TOKEN)
        .send()
        .await;

    match res {
        Ok(_) => console::log_1(&"Enrollment successful".into()),
        Err(e) => console::log_1(&format!("Enrollment failed: {:?}", e).into()),
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let courses = use_state(Vec::<Course>::new);
    let navigator = use_navigator().expect("Navigator not available");

    // Fetch all courses when the component mounts
    {
        let courses = courses.clone();
        use_effect_with((), move |_| {
            let courses = courses.clone();
            spawn_local(async move {
                match Request::get("http://localhost:5000/api/auth/get-all-courses")
                    .send()
                    .await
                {
                    Ok(resp) => match resp.json::<serde_json::Value>().await {
                        Ok(json) => {
                            if let Ok(parsed) =
                                serde_json::from_value::<Vec<Course>>(json["courses"].clone())
                            {
                                courses.set(parsed);
                            } else {
                                console::log_1(&"Failed to parse courses".into());
                            }
                        }
                        Err(e) => console::log_1(&format!("Failed to parse JSON: {:?}", e).into()),
                    },
                    Err(e) => console::log_1(&format!("Request failed: {:?}", e).into()),
                }
            });
            || ()
        });
    }

    // Handle "Enroll" button click
    let handle_enroll = {
        let navigator = navigator.clone();
        Callback::from(move |course: Course| {
            let navigator = navigator.clone();
            let course_id = course.course_id.clone();
            spawn_local(async move {
                enroll(course_id.clone()).await;
                navigator.push(&Route::UserCourse);
            });
        })
    };

    html! {
        <div class="p-4">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                { for courses.iter().map(|course| {
                    let course_clone = course.clone();
                    html! {
                        <div class="bg-white p-4 rounded shadow cursor-pointer hover:shadow-md transition-shadow duration-200">
                            <img
                                class="rounded mb-3"
                                width="500"
                                height="500"
                                src={course.thumbnail.clone()}
                                alt={course.course_name.clone()}
                            />
                            <h2 class="font-bold text-xl mb-2">{ &course.course_name }</h2>
                            <p class="text-gray-600">{ &course.description }</p>
                            <p class="mt-2 text-sm text-gray-400">{ &course.language }</p>
                            <button
                                class="bg-blue-500 hover:bg-blue-600 text-white rounded mt-3 px-5 py-2"
                                onclick={handle_enroll.reform(move |_| course_clone.clone())}
                            >
                                {"Enroll"}
                            </button>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}
