use imgui::*;

pub struct AppState {
    counter: isize,
}

impl AppState {
    pub fn new() -> Self {
        AppState { counter: 0 }
    }
}

pub fn update_and_render<'a>(ui: &Ui<'a>, app_state: &mut AppState) -> bool {
    ui.window(im_str!("Hello world"))
        .size((300.0, 200.0), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("こんにちは世界！"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos.0,
                mouse_pos.1
            ));
            ui.separator();
            let mut offset = 30.0;
            if ui.button(im_str!("-"), (30.0, 20.0)) {
                app_state.counter = app_state.counter.wrapping_sub(1);
            }
            offset += 30.0;
            ui.same_line(offset);
            ui.text(im_str!("{}", app_state.counter));
            offset += 30.0;
            ui.same_line(offset);
            if ui.button(im_str!("+"), (30.0, 20.0)) {
                app_state.counter = app_state.counter.wrapping_add(1);
            }
        });

    true
}
