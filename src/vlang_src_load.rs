#[inline]
pub(crate) fn keywords_v<'a>() -> &'a str {
    if cfg!(windows) {
        include_str!(r"vlang_src\keywords.v")
    } else {
        include_str!(r"vlang_src/keywords.v")
    }
}

