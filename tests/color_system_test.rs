use rich_rust::console::{ColorSystem, Console};

#[test]
fn test_color_system_standard_downsampling() {
    // RGB Red #FF0000 should downsample to ANSI Red (31) or Bright Red (91)
    // depending on the exact quantization, but definitely NOT 38;2 or 38;5
    let console = Console::capture().color_system(ColorSystem::Standard);
    console.print("[#ff0000]Hello[/]");

    let output = console.get_captured_output();

    // Check for standard codes
    // 31 is Red, 91 is BrightRed
    if !output.contains("\x1b[31m") && !output.contains("\x1b[91m") {
        panic!(
            "Output did not contain standard red/bright-red. Output: {:?}",
            output
        );
    }

    // Check that we DON'T have 256 color codes
    if output.contains("\x1b[38;5;") {
        panic!("Output contained 256-color code! Output: {:?}", output);
    }
}

#[test]
fn test_color_system_eightbit_downsampling() {
    let console = Console::capture().color_system(ColorSystem::EightBit);
    console.print("[#ff0000]Hello[/]");

    let output = console.get_captured_output();

    // Should NOT contain TrueColor codes
    assert!(
        !output.contains("\x1b[38;2;"),
        "Should not contain TrueColor codes"
    );

    // Should contain 256-color code
    if !output.contains("\x1b[38;5;") {
        panic!("Should contain 256-color code, got: {:?}", output);
    }
}

#[test]
fn test_color_system_truecolor() {
    let console = Console::capture().color_system(ColorSystem::TrueColor);
    console.print("[#ff0000]Hello[/]");

    let output = console.get_captured_output();

    // Should contain TrueColor code
    assert!(
        output.contains("\x1b[38;2;255;0;0m"),
        "Should contain TrueColor code: {:?}",
        output
    );
}

#[test]
fn test_color_system_no_color() {
    let console = Console::capture().color_system(ColorSystem::NoColor);
    console.print("[#ff0000]Hello[/]");

    let output = console.get_captured_output();

    // Should be plain text
    assert!(
        !output.contains("\x1b["),
        "Should not contain ANSI codes, got: {:?}",
        output
    );
    assert!(output.contains("Hello"), "Should contain content");
}
