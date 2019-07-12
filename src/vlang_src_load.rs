#[inline]
pub(crate) fn keywords_v<'a>() -> &'a str {
    include_str!(r"vlang_src/keywords.v")
}
