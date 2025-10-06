use crate::routes::Route;
use gloo_net::http::Request;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(AddCourse)]
pub fn add_course() -> Html {
    let course_name = use_state(|| "".to_string());
    let description = use_state(|| "".to_string());
    let language = use_state(|| "".to_string());
    let thumbnail = use_state(|| "".to_string());
    let titles = use_state(|| "".to_string());
    let videos = use_state(|| "".to_string());
    let navigator = use_navigator().unwrap();

    let onsubmit = {
        let course_name = course_name.clone();
        let description = description.clone();
        let thumbnail = thumbnail.clone();
        let language = language.clone();
        let titles = titles.clone();
        let videos = videos.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let token = window()
                .unwrap()
                .local_storage()
                .unwrap()
                .unwrap()
                .get_item("token")
                .unwrap_or_default()
                .unwrap_or_default();

            let course_name = (*course_name).clone();
            let description = (*description).clone();
            let language = (*language).clone();
            let thumbnail = (*thumbnail).clone();
            let course_titles: Vec<String> =
                (*titles).split(',').map(|s| s.trim().to_string()).collect();
            let course_videos: Vec<String> =
                (*videos).split(',').map(|s| s.trim().to_string()).collect();
            let navigator = navigator.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let _resp: serde_json::Value =
                    Request::post("http://localhost:5000/api/user/add-course")
                        .header("Authorization", &format!("{}", token))
                        .header("Content-Type", "application/json")
                        .body(
                            serde_json::json!({
                                "courseName": course_name,
                                "description": description,
                                "thumbnail":thumbnail,
                                "language": language,
                                "courseTitles": course_titles,
                                "courseVideos": course_videos
                            })
                            .to_string(),
                        )
                        .expect("Failed to create request")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                navigator.push(&Route::Home);
            });
        })
    };

    html! {
        <div class="flex justify-center p-4">
            <form class="bg-white p-6 rounded shadow-md w-full md:w-1/2" {onsubmit}>
                <h1 class="text-2xl font-bold mb-4">{"Add New Course"}</h1>
                <input type="text" placeholder="Course Name" class="border p-2 w-full mb-2"
                    value = {(*course_name).clone()}
                    oninput = {Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        course_name.set(input.value());
                    })}
                />
                <textarea placeholder="Description" class="border p-2 w-full mb-2"
                    value = {(*description).clone()}
                    oninput = {Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                        description.set(input.value());
                    })}
                />
                <input type="text" placeholder="Language" class="border p-2 w-full mb-2"
                    value = {(*language).clone()}
                    oninput = {Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        language.set(input.value());
                    })}
                />
                <input type="text" placeholder="Thumbnail link" class="border p-2 w-full mb-2"
                    value = {(*thumbnail).clone()}
                    oninput = {Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        thumbnail.set(input.value());
                    })}
                />
                <textarea placeholder="Course Titles (comma separated)" class="border p-2 w-full mb-2"
                    value = {(*titles).clone()}
                    oninput = {Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                        titles.set(input.value());
                    })}
                />
                <textarea placeholder="Course Videos (comma separated URLs)" class="border p-2 w-full mb-2"
                    value = {(*videos).clone()}
                    oninput = {Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                        videos.set(input.value());
                    })}
                />
                <button type="submit" class="bg-blue-500 text-white p-2 w-full rounded">{"Add Course"}</button>
            </form>
        </div>
    }
}
