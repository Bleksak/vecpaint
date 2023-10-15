use rui::*;

pub fn window_layout() -> impl View {
    vstack((
        "This is a test",
        rectangle().flex(),
        "This is another test",
        rectangle().flex()
    ))
}
