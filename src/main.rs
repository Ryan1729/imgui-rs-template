use app::update_and_render;
mod support_gfx;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    support_gfx::run("Some imgui-rs thing".to_owned(), CLEAR_COLOR, update_and_render);
}
