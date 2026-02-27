use std::fmt::{self, Write as _};

pub enum MoreLabel {
    Default,
}

pub fn list_to_string<T, F>(
    label: &str,
    items: &[T],
    max: usize,
    _more: MoreLabel,
    mut render_item: F,
) -> String
where
    F: FnMut(&mut String, usize, &T),
{
    let mut out = String::new();
    let _ = writeln!(&mut out, "{label}: {}", items.len());
    for (idx, item) in items.iter().enumerate().take(max) {
        render_item(&mut out, idx, item);
    }
    if items.len() > max {
        let rest = items.len() - max;
        let _ = writeln!(&mut out, "... and {rest} more");
    }
    out
}

pub fn info_display_to_string<F>(
    label: &str,
    uuid: &rac_protocol::Uuid16,
    fields: &[String],
    max: usize,
    format_uuid: F,
) -> String
where
    F: FnOnce(&rac_protocol::Uuid16) -> String,
{
    let mut out = String::new();
    let _ = writeln!(&mut out, "{label}: {}", format_uuid(uuid));
    for value in fields.iter().take(max) {
        let _ = writeln!(&mut out, "- {value}");
    }
    if fields.len() > max {
        let _ = writeln!(&mut out, "... and {} more", fields.len() - max);
    }
    out
}

pub fn write_trimmed(f: &mut fmt::Formatter<'_>, out: &str) -> fmt::Result {
    write!(f, "{}", out.trim_end())
}
