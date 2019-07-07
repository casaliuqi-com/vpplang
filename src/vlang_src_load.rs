#[inline]
pub(crate) fn keywords_v() -> String {
    include_str!(r"vlang_src\keywords.v").to_string()
}

