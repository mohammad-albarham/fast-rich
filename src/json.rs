//! JSON rendering with syntax highlighting.
//!
//! Provides beautiful JSON output with colors for keys, strings, numbers,
//! booleans, and null values.
//!
//! # Example
//!
//! ```no_run
//! use fast_rich::prelude::*;
//! use fast_rich::json::Json;
//!
//! let console = Console::new();
//! let json = Json::from_str(r#"{"name": "Alice", "age": 30}"#).unwrap();
//! console.print_renderable(&json);
//! ```

use crate::console::RenderContext;
use crate::highlighter::{Highlighter, JsonHighlighter};
use crate::renderable::{Renderable, Segment};
#[cfg(test)]
use crate::text::Overflow;
use crate::text::Text;
use serde::Serialize;
use serde_json::{self, Value};
use std::fmt;
use std::fs;
use std::path::Path;

/// Error type for JSON operations.
#[derive(Debug)]
pub enum JsonError {
    /// Failed to parse JSON
    Parse(serde_json::Error),
    /// Failed to serialize data
    Serialize(serde_json::Error),
    /// Failed to read file
    Io(std::io::Error),
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::Parse(e) => write!(f, "JSON parse error: {}", e),
            JsonError::Serialize(e) => write!(f, "JSON serialize error: {}", e),
            JsonError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for JsonError {}

impl From<std::io::Error> for JsonError {
    fn from(e: std::io::Error) -> Self {
        JsonError::Io(e)
    }
}

/// Indentation style for JSON output.
///
/// Matches Python rich's `indent` parameter which can be `None`, `int`, or `str`.
#[derive(Debug, Clone)]
pub enum JsonIndent {
    /// Compact output with no indentation or newlines
    Compact,
    /// Number of spaces for indentation
    Spaces(usize),
    /// Custom string for indentation (e.g., tab "\t")
    Custom(String),
}

impl Default for JsonIndent {
    fn default() -> Self {
        JsonIndent::Spaces(2)
    }
}

impl From<usize> for JsonIndent {
    fn from(n: usize) -> Self {
        if n == 0 {
            JsonIndent::Compact
        } else {
            JsonIndent::Spaces(n)
        }
    }
}

impl From<&str> for JsonIndent {
    fn from(s: &str) -> Self {
        JsonIndent::Custom(s.to_string())
    }
}

/// Options for JSON rendering, matching Python rich's JSON class.
#[derive(Debug, Clone)]
pub struct JsonOptions {
    /// Indentation style (default: 2 spaces)
    pub indent: JsonIndent,
    /// Enable syntax highlighting (default: true)
    pub highlight: bool,
    /// Sort object keys alphabetically (default: false)
    pub sort_keys: bool,
    /// Escape all non-ASCII characters to \uXXXX (default: false)
    pub ensure_ascii: bool,
    /// Disable word wrapping (default: true, matching Python rich)
    pub no_wrap: bool,
}

impl Default for JsonOptions {
    fn default() -> Self {
        JsonOptions {
            indent: JsonIndent::Spaces(2),
            highlight: true,
            sort_keys: false,
            ensure_ascii: false,
            no_wrap: true, // Match Python rich's behavior
        }
    }
}

/// A renderable that pretty-prints JSON with syntax highlighting.
///
/// # Example
///
/// ```no_run
/// use fast_rich::json::Json;
///
/// // From a JSON string
/// let json = Json::from_str(r#"{"key": "value"}"#).unwrap();
///
/// // From serializable data
/// use serde::Serialize;
/// #[derive(Serialize)]
/// struct User { name: String, age: u32 }
/// let user = User { name: "Alice".into(), age: 30 };
/// let json = Json::from_data(&user).unwrap();
///
/// // With options
/// let json = Json::from_str(r#"{"z": 1, "a": 2}"#)
///     .unwrap()
///     .sort_keys()
///     .indent(4)
///     .ensure_ascii();
///
/// // Compact output
/// let json = Json::from_str(r#"{"a": 1}"#).unwrap().compact();
///
/// // Tab indentation
/// let json = Json::from_str(r#"{"a": 1}"#).unwrap().indent_with("\t");
/// ```
#[derive(Debug, Clone)]
pub struct Json {
    /// The formatted and highlighted text
    text: Text,
    /// Original parsed value for re-rendering with different options
    value: Value,
    /// Current options
    options: JsonOptions,
}

impl Json {
    /// Create a JSON renderable from a JSON string.
    ///
    /// # Arguments
    ///
    /// * `json` - A valid JSON string
    ///
    /// # Returns
    ///
    /// Returns a `Json` instance or a `JsonError` if parsing fails.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(json: &str) -> Result<Self, JsonError> {
        Self::from_str_with_options(json, JsonOptions::default())
    }

    /// Create a JSON renderable from a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to a JSON file
    ///
    /// # Returns
    ///
    /// Returns a `Json` instance or a `JsonError` if reading/parsing fails.
    ///
    /// # Example
    /// ```no_run
    /// use fast_rich::json::Json;
    /// let json = Json::from_file("config.json").unwrap();
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, JsonError> {
        let content = fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    /// Create a JSON renderable from serializable data.
    ///
    /// # Arguments
    ///
    /// * `data` - Any type implementing `Serialize`
    ///
    /// # Returns
    ///
    /// Returns a `Json` instance or a `JsonError` if serialization fails.
    pub fn from_data<T: Serialize>(data: &T) -> Result<Self, JsonError> {
        let value = serde_json::to_value(data).map_err(JsonError::Serialize)?;
        Self::from_value(value, JsonOptions::default())
    }

    /// Create a JSON renderable with custom options.
    pub fn from_str_with_options(json: &str, options: JsonOptions) -> Result<Self, JsonError> {
        let value: Value = serde_json::from_str(json).map_err(JsonError::Parse)?;
        Self::from_value(value, options)
    }

    /// Create a JSON renderable from a serde_json Value.
    fn from_value(value: Value, options: JsonOptions) -> Result<Self, JsonError> {
        let text = Self::render_value(&value, &options);
        Ok(Json {
            text,
            value,
            options,
        })
    }

    /// Render the JSON value to styled Text.
    fn render_value(value: &Value, options: &JsonOptions) -> Text {
        // Sort keys if requested
        let value_to_render = if options.sort_keys {
            sort_json_keys(value)
        } else {
            value.clone()
        };

        // Format JSON
        let formatted = format_json(&value_to_render, &options.indent, options.ensure_ascii);

        // Apply highlighting
        let mut text = if options.highlight {
            let highlighter = JsonHighlighter::new();
            let spans = highlighter.highlight(&formatted);
            Text::from_spans(spans)
        } else {
            Text::plain(formatted)
        };

        // Apply no_wrap (match Python rich behavior)
        if options.no_wrap {
            text = text.no_wrap();
        }

        text
    }

    /// Re-render with updated options
    fn rerender(&self) -> Self {
        let text = Self::render_value(&self.value, &self.options);
        Json {
            text,
            value: self.value.clone(),
            options: self.options.clone(),
        }
    }

    /// Set custom indentation (number of spaces).
    ///
    /// Use `0` for compact output, or any positive number for that many spaces.
    ///
    /// # Example
    /// ```no_run
    /// use fast_rich::json::Json;
    /// let json = Json::from_str(r#"{"a": 1}"#).unwrap().indent(4);
    /// ```
    pub fn indent(mut self, spaces: usize) -> Self {
        self.options.indent = JsonIndent::from(spaces);
        self.rerender()
    }

    /// Set custom indentation using a string (e.g., tab).
    ///
    /// # Example
    /// ```no_run
    /// use fast_rich::json::Json;
    /// let json = Json::from_str(r#"{"a": 1}"#).unwrap().indent_with("\t");
    /// ```
    pub fn indent_with(mut self, indent_str: &str) -> Self {
        self.options.indent = JsonIndent::Custom(indent_str.to_string());
        self.rerender()
    }

    /// Output compact JSON (no indentation, single line).
    ///
    /// # Example
    /// ```no_run
    /// use fast_rich::json::Json;
    /// let json = Json::from_str(r#"{"a": 1, "b": 2}"#).unwrap().compact();
    /// // Output: {"a":1,"b":2}
    /// ```
    pub fn compact(mut self) -> Self {
        self.options.indent = JsonIndent::Compact;
        self.rerender()
    }

    /// Sort object keys alphabetically.
    ///
    /// # Example
    /// ```no_run
    /// use fast_rich::json::Json;
    /// let json = Json::from_str(r#"{"z": 1, "a": 2}"#).unwrap().sort_keys();
    /// // Output will have keys in order: "a", "z"
    /// ```
    pub fn sort_keys(mut self) -> Self {
        self.options.sort_keys = true;
        self.rerender()
    }

    /// Escape all non-ASCII characters to \uXXXX format.
    ///
    /// Useful when outputting to systems that don't handle Unicode well.
    ///
    /// # Example
    /// ```no_run
    /// use fast_rich::json::Json;
    /// let json = Json::from_str(r#"{"emoji": "😀"}"#).unwrap().ensure_ascii();
    /// // "😀" becomes "\ud83d\ude00"
    /// ```
    pub fn ensure_ascii(mut self) -> Self {
        self.options.ensure_ascii = true;
        self.rerender()
    }

    /// Disable syntax highlighting.
    ///
    /// # Example
    /// ```no_run
    /// use fast_rich::json::Json;
    /// let json = Json::from_str(r#"{"a": 1}"#).unwrap().no_highlight();
    /// ```
    pub fn no_highlight(mut self) -> Self {
        self.options.highlight = false;
        self.rerender()
    }

    /// Enable word wrapping (by default, wrapping is disabled).
    ///
    /// # Example
    /// ```no_run
    /// use fast_rich::json::Json;
    /// let json = Json::from_str(r#"{"a": 1}"#).unwrap().wrap();
    /// ```
    pub fn wrap(mut self) -> Self {
        self.options.no_wrap = false;
        self.rerender()
    }

    /// Get the plain text content (useful for testing).
    pub fn plain_text(&self) -> String {
        self.text.plain_text()
    }
}

impl Renderable for Json {
    fn render(&self, context: &RenderContext) -> Vec<Segment> {
        self.text.render(context)
    }
}

/// Format a JSON value with custom indentation and ASCII escaping.
fn format_json(value: &Value, indent: &JsonIndent, ensure_ascii: bool) -> String {
    match indent {
        JsonIndent::Compact => {
            // Compact output - no whitespace
            let result = serde_json::to_string(value).unwrap_or_default();
            if ensure_ascii {
                escape_non_ascii(&result)
            } else {
                result
            }
        }
        JsonIndent::Spaces(n) => {
            let indent_bytes = " ".repeat(*n).into_bytes();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
            let mut buf = Vec::new();
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            value.serialize(&mut ser).ok();
            let result = String::from_utf8(buf).unwrap_or_default();

            if ensure_ascii {
                escape_non_ascii(&result)
            } else {
                result
            }
        }
        JsonIndent::Custom(s) => {
            let indent_bytes = s.as_bytes().to_vec();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_bytes);
            let mut buf = Vec::new();
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            value.serialize(&mut ser).ok();
            let result = String::from_utf8(buf).unwrap_or_default();

            if ensure_ascii {
                escape_non_ascii(&result)
            } else {
                result
            }
        }
    }
}

/// Escape non-ASCII characters to \uXXXX format.
fn escape_non_ascii(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii() {
            result.push(c);
        } else {
            // Encode as \uXXXX (or surrogate pairs for chars > 0xFFFF)
            let code = c as u32;
            if code <= 0xFFFF {
                result.push_str(&format!("\\u{:04x}", code));
            } else {
                // Need surrogate pair
                let code = code - 0x10000;
                let high = 0xD800 + (code >> 10);
                let low = 0xDC00 + (code & 0x3FF);
                result.push_str(&format!("\\u{:04x}\\u{:04x}", high, low));
            }
        }
    }
    result
}

/// Recursively sort JSON object keys alphabetically.
fn sort_json_keys(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut sorted: Vec<_> = map.iter().collect();
            sorted.sort_by(|a, b| a.0.cmp(b.0));
            let sorted_map: serde_json::Map<String, Value> = sorted
                .into_iter()
                .map(|(k, v)| (k.clone(), sort_json_keys(v)))
                .collect();
            Value::Object(sorted_map)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(sort_json_keys).collect()),
        _ => value.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_json_from_str() {
        let json = Json::from_str(r#"{"name": "Alice"}"#);
        assert!(json.is_ok());
    }

    #[test]
    fn test_json_invalid() {
        let json = Json::from_str("not valid json");
        assert!(json.is_err());
    }

    #[test]
    fn test_json_from_data() {
        #[derive(Serialize)]
        struct User {
            name: String,
            age: u32,
        }

        let user = User {
            name: "Bob".to_string(),
            age: 25,
        };
        let json = Json::from_data(&user);
        assert!(json.is_ok());
        let text = json.unwrap().plain_text();
        assert!(text.contains("Bob"));
        assert!(text.contains("25"));
    }

    #[test]
    fn test_json_sort_keys() {
        let json = Json::from_str(r#"{"z": 1, "a": 2, "m": 3}"#)
            .unwrap()
            .sort_keys();
        let text = json.plain_text();
        let a_pos = text.find("\"a\"").unwrap();
        let m_pos = text.find("\"m\"").unwrap();
        let z_pos = text.find("\"z\"").unwrap();
        assert!(a_pos < m_pos);
        assert!(m_pos < z_pos);
    }

    #[test]
    fn test_json_nested() {
        let json = Json::from_str(r#"{"user": {"name": "Alice", "scores": [1, 2, 3]}}"#);
        assert!(json.is_ok());
    }

    #[test]
    fn test_json_types() {
        let json = Json::from_str(
            r#"{"str": "hello", "num": 42, "float": 3.14, "bool": true, "null": null}"#,
        );
        assert!(json.is_ok());
    }

    #[test]
    fn test_json_indent_spaces() {
        let json = Json::from_str(r#"{"a": 1}"#).unwrap().indent(4);
        let text = json.plain_text();
        // With 4-space indent, should have "    " before "a"
        assert!(text.contains("    \"a\""));
    }

    #[test]
    fn test_json_indent_tab() {
        let json = Json::from_str(r#"{"a": 1}"#).unwrap().indent_with("\t");
        let text = json.plain_text();
        // With tab indent, should have "\t" before "a"
        assert!(text.contains("\t\"a\""));
    }

    #[test]
    fn test_json_compact() {
        let json = Json::from_str(r#"{"a": 1, "b": 2}"#).unwrap().compact();
        let text = json.plain_text();
        // Compact output should have no newlines or extra spaces
        assert!(!text.contains('\n'));
        assert_eq!(text, r#"{"a":1,"b":2}"#);
    }

    #[test]
    fn test_json_ensure_ascii() {
        let json = Json::from_str(r#"{"emoji": "😀"}"#).unwrap().ensure_ascii();
        let text = json.plain_text();
        // Should have escaped emoji
        assert!(text.contains("\\u"));
        assert!(!text.contains("😀"));
    }

    #[test]
    fn test_json_no_highlight() {
        let json = Json::from_str(r#"{"a": 1}"#).unwrap().no_highlight();
        // Should still work, just without colors
        assert!(json.plain_text().contains("\"a\""));
    }

    #[test]
    fn test_json_no_wrap_default() {
        let json = Json::from_str(r#"{"a": 1}"#).unwrap();
        // By default, no_wrap should be true (Overflow::Visible)
        assert_eq!(json.text.overflow, Overflow::Visible);
    }

    #[test]
    fn test_json_wrap() {
        let json = Json::from_str(r#"{"a": 1}"#).unwrap().wrap();
        // After calling wrap(), overflow should be Wrap
        assert_eq!(json.text.overflow, Overflow::Wrap);
    }

    #[test]
    fn test_json_from_file() {
        // Create a temp file with JSON content
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"{{"name": "test", "value": 42}}"#).unwrap();

        let json = Json::from_file(temp_file.path());
        assert!(json.is_ok());
        let text = json.unwrap().plain_text();
        assert!(text.contains("test"));
        assert!(text.contains("42"));
    }

    #[test]
    fn test_json_from_file_not_found() {
        let json = Json::from_file("/nonexistent/path/to/file.json");
        assert!(json.is_err());
    }
}
