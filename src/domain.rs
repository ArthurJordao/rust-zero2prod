use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, ()> {
        let is_blank = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_chars = HashSet::from(['/', '(', ')', '"', '"', '<', '>', '\\', '{', '}']);
        let contains_forbidden_characters = s.chars().any(|g| forbidden_chars.contains(&g));
        if is_blank || is_too_long || contains_forbidden_characters {
            Err(())
        } else {
            Ok(Self(s))
        }
    }
}
