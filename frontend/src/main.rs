use app::App;

mod app;
mod bodies;
mod responses;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
