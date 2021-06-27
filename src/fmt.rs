
pub fn get_session_name(user: &str, host: &str) -> String {
    format!("{}@{}", user, host)
}

pub fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
    }
}
