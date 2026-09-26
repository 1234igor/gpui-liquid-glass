fn main() {
    if std::env::args().any(|argument| argument == "window") {
        gpui_liquid_glass::window_glass::launch_window_glass();
    } else {
        gpui_liquid_glass::liquid_glass::launch_media_player();
    }
}
