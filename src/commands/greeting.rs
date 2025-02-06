pub fn welcome_msg() -> String {
    use crate::{ENV, colors::apply_color};

    let env = ENV.lock().unwrap();
    let date = chrono::Local::now();

    let mut msg = format!(
        "Welcome {}!\n",
        apply_color("magenta_bright", &env.git_name),
    );
    msg.push_str(
        &format!(
            "Today is {}.\n\n",
            &apply_color("green_bright", &date.format("%A, %B %d, %Y").to_string()),
        ),
    );
    msg
}
