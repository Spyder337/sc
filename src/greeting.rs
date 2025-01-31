pub fn welcome_msg() -> String {
    use crate::{ENV, colors::apply_color};

    let env = ENV.lock().unwrap();
    let date = chrono::Local::now();

    let mut msg = format!(
        "Welcome {}!\n",
        apply_color("magenta", env.git_name.as_str()),
    );
    msg.push_str(format!(
        "Today is {}.\n",
        apply_color("green", date.format("%A, %B %d, %Y").to_string().as_str()),
    ).as_str());
    msg
}