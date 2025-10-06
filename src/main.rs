use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod routes;


use components::{AddCourse, CourseDetail, Home, Login, Register, UserCourse};
use routes::Route;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::Login => html! {<Login/>},
        Route::Register => html! {<Register/>},
        Route::AddCourse => html! {<AddCourse/>},
        Route::UserCourse => html! {<UserCourse/>},
        Route::CourseDetail { id } => html! {<CourseDetail id={id}/>},
        // Route::NotFound => html!{<h1>{"Page Not Found"}</h1>},
    }
}

#[function_component(Main)]
fn app() -> Html {


    html! {
        <BrowserRouter>

            <nav class="bg-indigo-600 p-4 shadow-md">
                <div class="max-w-7xl mx-auto flex items-center justify-between">
                    // <!-- Logo Section -->
                    <div class="text-white text-2xl font-semibold">
                        <div href="/" class="flex items-center space-x-2">
                            <Link<Route> to={Route::Home} >
                                <span class="text-yellow-400">{"L"}</span>
                                <span class="text-white">{"MS"}</span>
                            </Link<Route>>
                        </div>
                    </div>
                    // <!-- Navigation Links -->
                    <div class="hidden md:flex space-x-8 text-white">
                        <div class="hover:bg-indigo-700 p-2 rounded"><Link<Route> to={Route::Home} >{"Home"}</Link<Route>></div>
                            <div class="hover:bg-indigo-700 p-2 rounded"><Link<Route> to={Route::UserCourse} >{"my courses"}</Link<Route>></div>
                            <div class="hover:bg-indigo-700 p-2 rounded"><Link<Route> to={Route::AddCourse} >{"add course"}</Link<Route>></div>
                            // <div class="hover:bg-indigo-700 p-2 rounded"><Link<Route> to={Route::Login} >{"Login"}</Link<Route>></div>
                            // <div class="hover:bg-indigo-700 p-2 rounded"><Link<Route> to={Route::Register} >{"Register"}</Link<Route>></div>
                    </div>
                </div>
            </nav>
            <Switch<Route> render={switch} />
        </BrowserRouter>

    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}
