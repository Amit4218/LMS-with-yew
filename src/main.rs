// mod types;
mod components;


use yew::prelude::*;
use components::video::{Video, videos};
use components::test::hello;

#[function_component(App)]
fn app() -> Html {

//     let videos: Vec<Video> = vec![
//     Video {
//         id: 1,
//         title: "Building and breaking things".to_string(),
//         speaker: "John Doe".to_string(),
//         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//     },
//     Video {
//         id: 2,
//         title: "The development process".to_string(),
//         speaker: "Jane Smith".to_string(),
//         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//     },
//     Video {
//         id: 3,
//         title: "The Web 7.0".to_string(),
//         speaker: "Matt Miller".to_string(),
//         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//     },
//     Video {
//         id: 4,
//         title: "Mouseless development".to_string(),
//         speaker: "Tom Jerry".to_string(),
//         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
//     },
// ];

let videos_list = videos();
let videos_html = videos_list.iter().map(|video: &Video| html! {
    <>
    <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
    <a href={video.url.clone()} target="_blank" >{video.title.clone()}</a>
    </>
}).collect::<Html>();
    
        html! {
    <>
        <h1>{ format!("{} RustConf Explorer", hello()) }</h1>
        <div>
            <h3>{"Videos to watch"}</h3>
            {videos_html}
        </div>
        <div>
            <h3>{ "John Doe: Building and breaking things" }</h3>
            <img src="https://placehold.co/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    </>
}
    
}

fn main() {
    yew::Renderer::<App>::new().render();
}