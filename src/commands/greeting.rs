use crate::Colorize;

pub fn welcome_msg() -> String {
    use crate::ENV;

    let env = ENV.lock().unwrap();
    let date = chrono::Local::now();

    let mut msg = format!(
        "Welcome {}!\n",
        &env.git_name.magenta_bright(),
    );
    msg.push_str(&format!(
        "Today is {}.\n\n",
        &date.format("%A, %B %d, %Y").to_string().green()),
    );
    msg
}
