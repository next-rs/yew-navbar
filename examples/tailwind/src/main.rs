mod app;
mod pages;
mod router;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
