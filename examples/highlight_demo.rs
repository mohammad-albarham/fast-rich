//! Automatic highlighting of printed strings (Rich's `highlight=True`).
//!
//! Run in a terminal:          cargo run --example highlight_demo
//! Also save an HTML preview:  cargo run --example highlight_demo -- --html highlight.html

use fast_rich::Console;

const LINES: &[&str] = &[
    "Server 192.168.1.10 returned 404 in 3.5s, retry=True, cache=None",
    "GET https://api.example.com/v1/users?id=42 -> 200 OK",
    "Reading /var/log/app.log (12 MB), id=550e8400-e29b-41d4-a716-446655440000",
    "Result: Some(Point { x: 1, y: -2 }), ok: true, failed: false",
    "data=[1, 2.5, 'three', None] <Response [200]> loading...",
    "ipv6 fe80::1, mac 00-1A-2B-3C-4D-5E, hex 0xff, complex 3+4j",
    "Markup still wins: [bold red]42[/] vs 42",
];

fn main() {
    let html_path = std::env::args().skip_while(|a| a != "--html").nth(1);

    let console = Console::new().record(html_path.is_some());
    console.rule("highlight on (default)");
    for line in LINES {
        console.println(line);
    }

    let plain = Console::new().highlight(false);
    plain.rule("highlight(false)");
    plain.println(LINES[0]);

    if let Some(path) = html_path {
        console.save_html(&path).expect("write HTML");
        println!("\nSaved {path}");
    }
}
