pub fn alert_message(message: &str) {
    let window = gloo_utils::window();
    window
        .alert_with_message(message)
        .expect("Could not get the window context. Are you running in a browser?");
}
