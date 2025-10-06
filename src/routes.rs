use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/my-courses")]
    UserCourse,
    #[at("/course/:id")]
    CourseDetail { id: String },
    #[at("/add-course")]
    AddCourse,
    // #[not_found],
    // NotFound,
    #[at("/")]
    Home,
}
