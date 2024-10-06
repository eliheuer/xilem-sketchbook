use xilem::{
    view::{button, flex, label, Axis},
    EventLoop, WidgetView, Xilem,
};

struct AppState {
    count: i32,
}

fn app_logic(state: &mut AppState) -> impl WidgetView<AppState> {
    flex((
        button("-", |state: &mut AppState| state.count -= 1),
        label(format!("Count: {}", state.count)).text_size(64.0),
        button("+", |state: &mut AppState| state.count += 1),
    ))
    .direction(Axis::Vertical)
}

fn main() {
    let app = Xilem::new(AppState { count: 0 }, app_logic);
    app.run_windowed(EventLoop::with_user_event(), "My Xilem App".into())
        .unwrap();
}
