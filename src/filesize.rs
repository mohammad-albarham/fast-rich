//! Filesize formatting utilities.

const SUFFIXES: [&str; 9] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
const BINARY_SUFFIXES: [&str; 9] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

/// Convert a size in bytes to a human readable string using decimal suffixes (kB, MB).
pub fn decimal(size: u64) -> String {
    format_size(size, 1000.0, &SUFFIXES)
}

/// Convert a size in bytes to a human readable string using binary suffixes (KiB, MiB).
pub fn binary(size: u64) -> String {
    format_size(size, 1024.0, &BINARY_SUFFIXES)
}

fn format_size(size: u64, divisor: f64, suffixes: &[&str]) -> String {
    if size == 0 {
        return format!("0 {}", suffixes[0]);
    }

    let size_f = size as f64;
    let i = (size_f.log(divisor).floor() as usize).min(suffixes.len() - 1);

    if i == 0 {
        format!("{} {}", size, suffixes[0])
    } else {
        let val = size_f / divisor.powi(i as i32);
        format!("{:.1} {}", val, suffixes[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        assert_eq!(decimal(0), "0 B");
        assert_eq!(decimal(500), "500 B");
        assert_eq!(decimal(1000), "1.0 kB");
        assert_eq!(decimal(1500), "1.5 kB");
        assert_eq!(decimal(1_000_000), "1.0 MB");
    }

    #[test]
    fn test_binary() {
        assert_eq!(binary(0), "0 B");
        assert_eq!(binary(1024), "1.0 KiB");
        assert_eq!(binary(1536), "1.5 KiB");
        assert_eq!(binary(1_048_576), "1.0 MiB");
    }
}
