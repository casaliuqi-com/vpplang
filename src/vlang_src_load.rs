#[inline]
#[cfg(target_os = "windows")]
pub(crate) fn keywords_v<'a>() -> &'a str {
    include_str!(r"vlang_src\keywords.v")
}

#[inline]
#[cfg(not(target_os = "windows"))]
pub(crate) fn keywords_v<'a>() -> &'a str {
    include_str!(r"vlang_src/keywords.v")
}
