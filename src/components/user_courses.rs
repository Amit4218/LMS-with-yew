use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[derive(Clone, Deserialize, PartialEq, Debug)]
pub struct UserCourses {
    #[serde(rename = "userCourseId")]
    pub user_course_id: String,
    #[serde(rename = "courseId")]
    pub course_id: String,
    #[serde(rename = "courseName")]
    pub course_name: String,
    pub thumbnail: String,
    pub description: String,
    pub language: String,
}

#[derive(Deserialize, Debug)]
struct UserCoursesResponse {
    #[serde(rename = "Courses")] // Fix here
    courses: Vec<UserCourses>,
}

const TOKEN: &str = "";

#[function_component(UserCourse)]
pub fn user_courses() -> Html {
    let courses = use_state(Vec::<UserCourses>::new);
    let navigator = use_navigator().expect("Navigator not available");

    {
        let courses_for_effect = courses.clone();

        use_effect_with((), move |_| {
            let courses = courses_for_effect.clone();

            spawn_local(async move {
                match Request::get("http://localhost:5000/api/user/user-courses")
                    .header("Authorization", TOKEN)
                    .send()
                    .await
                {
                    Ok(resp) => match resp.json::<UserCoursesResponse>().await {
                        Ok(parsed) => {
                            console::log_1(&format!("Parsed courses: {:?}", parsed).into());
                            courses.set(parsed.courses);
                        }
                        Err(e) => console::log_1(&format!("Failed to parse JSON: {:?}", e).into()),
                    },
                    Err(e) => console::log_1(&format!("Request failed: {:?}", e).into()),
                }
            });

            || ()
        });
    }

    html! {
        <div class="p-4">
            <h1 class="text-2xl font-bold mb-4">{"My Courses"}</h1>

            {
                if courses.is_empty() {
                    html! {
                        <p class="text-gray-500">{"Loading your courses..."}</p>
                    }
                } else {
                    html! {
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                            { for courses.iter().map(|course| {
                                let navigator = navigator.clone();
                                let course_id = course.course_id.clone();

                                html! {
                                    <div
                                        onclick={Callback::from(move |_| {
                                            navigator.push(&Route::CourseDetail { id: course_id.clone() });
                                        })}
                                        class="bg-white p-4 rounded shadow cursor-pointer hover:shadow-md transition-shadow duration-200"
                                    >
                                        <img
                                            class="rounded mb-3 w-full object-cover"
                                            width="500"
                                            height="500"
                                            src={course.thumbnail.clone()}
                                            alt={course.course_name.clone()}
                                        />
                                        <h2 class="font-bold text-xl mb-2">{ &course.course_name }</h2>
                                        <p class="text-gray-600 line-clamp-3">{ &course.description }</p>
                                        <p class="mt-2 text-sm text-gray-400 italic">{ &course.language }</p>
                                    </div>
                                }
                            })}
                        </div>
                    }
                }
            }
        </div>
    }
}
