use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

const TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiJjbWdkbjdlcjkwMDAwb3lhZmwxeXVoZ2FyIiwiZW1haWwiOiJ0ZXN0QGdtYWlsLmNvbSIsInNlc3Npb25JZCI6ImNtZ2RyamgyMzAwMDBveTdkbXhlazlhcXEiLCJpYXQiOjE3NTk2NzI0NDUsImV4cCI6MTc2MDI3NzI0NX0.y6ESvU8pRaFmWlSwxemNHkYpfWwtA_fSYioz2GWqeHE";


#[derive(Clone, Deserialize, PartialEq, Debug)]
struct UserCourses {
    #[serde(rename = "userCourseId")]
    user_course_id: String,
    #[serde(rename = "courseId")]
    course_id: String,
    #[serde(rename = "courseName")]
    course_name: String,
    thumbnail: String,
    description: String,
    language: String,
}


#[function_component(CourseDetail)]
pub fn course_details() -> Html {
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
                    Ok(resp) => match resp.json::<serde_json::Value>().await {
                        Ok(json) => {
                            if let Ok(parsed) =
                                serde_json::from_value::<Vec<UserCourses>>(json["Courses"].clone())
                            {
                                courses.set(parsed);
                            } else {
                                console::log_1(&"Failed to parse courses".into());
                            }
                        }
                        Err(e) => console::log_1(
                            &format!("Failed to parse JSON: {:?}", e).into(),
                        ),
                    },
                    Err(e) => console::log_1(&format!("Request failed: {:?}", e).into()),
                }
            });
            || ()
        });
    }

    html! {
        <div class="p-4">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                { for courses.iter().map(|course| {
                    html! {
                        <div onclick={Callback::} >
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
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}