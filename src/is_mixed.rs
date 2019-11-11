//! Test if `input` contains a mix of [Romaji](https://en.wikipedia.org/wiki/Romaji) and [Kana](https://en.wikipedia.org/wiki/Kana), defaults to pass through [Kanji](https://en.wikipedia.org/wiki/Kanji)
//!
//! # Examples
//! ```
//! use wana_kana::is_mixed::*;
//! use wana_kana::Options;
//! assert_eq!(is_mixed("Aア"), true);
//! assert_eq!(is_mixed("Aあ"), true);
//! assert_eq!(is_mixed("Aあア"), true);
//! assert_eq!(is_mixed("２あア"), false);
//! assert_eq!(is_mixed("お腹A"), true);
//! assert_eq!(is_mixed_pass_kanji("Abあア", true), true);
//! assert_eq!(is_mixed_pass_kanji("お腹A", true), true);
//! assert_eq!(is_mixed_pass_kanji("お腹A", false), false);
//! assert_eq!(is_mixed_pass_kanji("ab", true), false);
//! assert_eq!(is_mixed_pass_kanji("あア", true), false);
//! ```

use crate::is_hiragana::is_hiragana;
use crate::is_kanji::*;
use crate::is_katakana::is_katakana;
use crate::is_romaji::is_romaji;

pub fn is_mixed(input: &str) -> bool {
    is_mixed_pass_kanji(input, true)
}

pub fn is_mixed_pass_kanji(input: &str, pass_kanji: bool) -> bool {
    let mut has_kanji = false;
    if !pass_kanji {
        has_kanji = input.chars().any(|c| is_kanji(&c.to_string()));
    }
    return (input.chars().any(|c| is_hiragana(&c.to_string())) || input.chars().any(|c| is_katakana(&c.to_string())))
        && input.chars().any(|c| is_romaji(&c.to_string()))
        && !has_kanji;
}
