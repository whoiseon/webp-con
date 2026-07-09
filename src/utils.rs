use std::path::Path;

pub fn is_image_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => {
            match ext.to_str() {
                Some(ext_str) => {
                    const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "bmp"];
                    IMAGE_EXTENSIONS.iter().any(|ext| ext.eq_ignore_ascii_case(ext_str))
                }
                None => false,
            }
        },
        None => false,
    }
}

pub fn format_bytes(bytes: u64) -> String {
    let bytes = bytes as f64;
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let (value, unit) = if bytes < KB {
        (bytes, "B")
    } else if bytes < MB {
        (bytes / KB, "KB")
    } else if bytes < GB {
        (bytes / MB, "MB")
    } else if bytes < TB {
        (bytes / GB, "GB")
    } else {
        (bytes / TB, "TB")
    };

    format!("{:.2}{}", value, unit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_bytes_as_b() {
        assert_eq!(format_bytes(500), "500.00B");
    }

    #[test]
    fn formats_bytes_as_kb() {
        assert_eq!(format_bytes(1024), "1.00KB");
        assert_eq!(format_bytes(1536), "1.50KB");
    }

    #[test]
    fn formats_bytes_as_mb() {
        assert_eq!(format_bytes(1024 * 1024), "1.00MB");
    }

    #[test]
    fn formats_bytes_as_gb() {
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00GB");
    }
}