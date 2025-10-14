/// HTML parsing utilities for centered images and other HTML blocks
use crate::models::Alignment;

/// Parse HTML block to extract image with alignment and width
/// Returns (image_src, alignment, width) if found
///
/// Handles patterns like:
/// ```html
/// <p align="center">
/// <img src="image.png" width="400">
/// </p>
/// ```
pub fn parse_html_image(html: &str) -> Option<(String, Option<Alignment>, Option<f32>)> {
    // Simple regex-free parser for <p align="..."><img src="..." width="..."></p>
    let html = html.trim();

    // Check if it starts with <p and contains align attribute
    if !html.starts_with("<p") {
        return None;
    }

    // Extract alignment from <p align="center|left|right">
    let alignment = parse_alignment(html);

    // Extract src and width from <img> tag
    parse_img_attributes(html, alignment)
}

/// Extract alignment attribute from HTML paragraph tag
fn parse_alignment(html: &str) -> Option<Alignment> {
    if html.contains("align=\"center\"") || html.contains("align='center'") {
        Some(Alignment::Center)
    } else if html.contains("align=\"right\"") || html.contains("align='right'") {
        Some(Alignment::Right)
    } else if html.contains("align=\"left\"") || html.contains("align='left'") {
        Some(Alignment::Left)
    } else {
        None
    }
}

/// Extract src and width attributes from <img> tag
fn parse_img_attributes(
    html: &str,
    alignment: Option<Alignment>,
) -> Option<(String, Option<Alignment>, Option<f32>)> {
    let img_start = html.find("<img")?;
    let img_section = &html[img_start..];

    // Extract src attribute
    let src = extract_attribute(img_section, "src")?;

    // Extract width attribute (optional)
    let width = extract_attribute(img_section, "width").and_then(|w| w.parse::<f32>().ok());

    Some((src, alignment, width))
}

/// Extract a single attribute value from HTML tag
/// Handles both single and double quotes
fn extract_attribute(html: &str, attr_name: &str) -> Option<String> {
    let attr_pattern_double = format!("{}=\"", attr_name);
    let attr_pattern_single = format!("{}='", attr_name);

    let (attr_start, quote_char) = if let Some(pos) = html.find(&attr_pattern_double) {
        (pos, '"')
    } else if let Some(pos) = html.find(&attr_pattern_single) {
        (pos, '\'')
    } else {
        return None;
    };

    let value_start = attr_start + attr_name.len() + 2; // name="
    let remaining = &html[value_start..];
    let value_end = remaining.find(quote_char)?;

    Some(remaining[..value_end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_centered_image_with_width() {
        let html = r#"<p align="center">
<img src="test.png" width="400">
</p>"#;

        let result = parse_html_image(html);
        assert!(result.is_some());

        let (src, alignment, width) = result.unwrap();
        assert_eq!(src, "test.png");
        assert!(matches!(alignment, Some(Alignment::Center)));
        assert_eq!(width, Some(400.0));
    }

    #[test]
    fn test_parse_image_without_alignment() {
        let html = r#"<p>
<img src="image.jpg">
</p>"#;

        let result = parse_html_image(html);
        assert!(result.is_some());

        let (src, alignment, width) = result.unwrap();
        assert_eq!(src, "image.jpg");
        assert!(alignment.is_none());
        assert!(width.is_none());
    }

    #[test]
    fn test_parse_right_aligned_image() {
        let html = r#"<p align="right"><img src="pic.png"></p>"#;

        let result = parse_html_image(html);
        assert!(result.is_some());

        let (src, alignment, _) = result.unwrap();
        assert_eq!(src, "pic.png");
        assert!(matches!(alignment, Some(Alignment::Right)));
    }

    #[test]
    fn test_parse_with_single_quotes() {
        let html = r#"<p align='center'><img src='test.png' width='200'></p>"#;

        let result = parse_html_image(html);
        assert!(result.is_some());

        let (src, _, width) = result.unwrap();
        assert_eq!(src, "test.png");
        assert_eq!(width, Some(200.0));
    }

    #[test]
    fn test_non_paragraph_html() {
        let html = "<div><img src='test.png'></div>";

        let result = parse_html_image(html);
        assert!(result.is_none());
    }

    #[test]
    fn test_missing_img_tag() {
        let html = "<p align='center'>No image here</p>";

        let result = parse_html_image(html);
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_attribute() {
        let html = r#"<img src="test.png" alt="Test">"#;

        assert_eq!(extract_attribute(html, "src"), Some("test.png".to_string()));
        assert_eq!(extract_attribute(html, "alt"), Some("Test".to_string()));
        assert_eq!(extract_attribute(html, "missing"), None);
    }
}
