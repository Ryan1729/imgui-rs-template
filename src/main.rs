use app;
use imgui_support_gfx;

const CLEAR_COLOR: [f32; 4] = [0.0625, 0.0625, 0.0625, 1.0];

fn main() {
    imgui_support_gfx::run(
        "Some imgui-rs thing".to_owned(),
        CLEAR_COLOR,
        app::update_and_render,
    );
}
