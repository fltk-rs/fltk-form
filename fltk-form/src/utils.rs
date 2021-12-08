use color_maps::html::HTML_MAP;

pub(crate) fn color_map_adaptor(html_name: &str) -> String {
    if html_name.starts_with('#') {
        return html_name.to_string();
    }
    if let Some(col) = HTML_MAP.get(html_name) {
        format!("#{:02x}{:02x}{:02x}", col.0, col.1, col.2)
    } else {
        html_name.to_string()
    }
}