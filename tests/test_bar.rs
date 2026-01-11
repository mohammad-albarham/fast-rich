//! Integration tests for bar charts

use fast_rich::bar::{BarChart, BarData};
use fast_rich::console::RenderContext;
use fast_rich::renderable::Renderable;
use fast_rich::style::Color;

#[test]
fn test_bar_chart_creation() {
    let mut chart = BarChart::new();
    chart.bar("Test", 50.0);

    let context = RenderContext {
        width: 60,
        height: None,
    };
    let segments = chart.render(&context);

    assert_eq!(segments.len(), 1);
}

#[test]
fn test_bar_data() {
    let bar = BarData::new("Label", 100.0);
    assert_eq!(bar.label, "Label");
    assert_eq!(bar.value, 100.0);
    assert!(bar.color.is_none());
}

#[test]
fn test_bar_data_with_color() {
    let bar = BarData::new("Colored", 75.0).color(Color::Red);
    assert_eq!(bar.label, "Colored");
    assert_eq!(bar.color, Some(Color::Red));
}

#[test]
fn test_multiple_bars() {
    let mut chart = BarChart::new();
    chart.bar("First", 25.0);
    chart.bar("Second", 50.0);
    chart.bar("Third", 75.0);

    let context = RenderContext {
        width: 60,
        height: None,
    };
    let segments = chart.render(&context);

    assert_eq!(segments.len(), 3);
}

#[test]
fn test_empty_chart() {
    let chart = BarChart::new();
    let context = RenderContext {
        width: 60,
        height: None,
    };
    let segments = chart.render(&context);

    assert_eq!(segments.len(), 1); // Empty line
}

#[test]
fn test_bar_chart_width() {
    let chart = BarChart::new().width(40);
    // Width is set, verified by compilation
    let _ = chart;
}

#[test]
fn test_bar_chart_character() {
    let chart = BarChart::new().bar_char('▓');
    // Character is set, verified by compilation
    let _ = chart;
}

#[test]
fn test_bar_chart_show_values() {
    let chart = BarChart::new().show_values(false);
    // show_values is set, verified by compilation
    let _ = chart;
}
