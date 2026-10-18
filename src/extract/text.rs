use scraper::ElementRef;

/// All descendant text, whitespace-normalised and capped at `max_chars`.
pub(super) fn text_of(el: ElementRef<'_>, max_chars: usize) -> String {
    let mut out = String::new();
    for word in el.text().flat_map(str::split_whitespace) {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(word);
    }

    if let Some((idx, _)) = out.char_indices().nth(max_chars) {
        out.truncate(idx);
        out.push('…');
    }
    out
}
