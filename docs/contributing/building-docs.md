# Building the Documentation

This guide explains how to build and contribute to the Fast-Rich documentation.

## Prerequisites

Install the required Python packages:

```bash
pip install mkdocs-material pymdown-extensions
```

Or with a requirements file:

```bash
pip install -r docs-requirements.txt
```

---

## Local Development

### Serve Locally

Start the development server with live reload:

```bash
mkdocs serve
```

Open [http://127.0.0.1:8000](http://127.0.0.1:8000) in your browser.

Changes to documentation files will automatically reload.

### Build Static Site

Generate the static HTML site:

```bash
mkdocs build
```

Output is in the `site/` directory.

---

## Documentation Structure

```
docs/
├── index.md                 # Landing page
├── getting-started.md       # Installation and basics
├── guides/                  # Feature guides
│   ├── index.md            # Guides overview
│   ├── text-styles.md      # Text and styling
│   ├── console.md          # Console usage
│   ├── tables.md           # Tables
│   └── ...
├── rust_examples.md         # Example programs
├── tutorial_dashboard.md    # Dashboard tutorial
├── reference/              # API reference
│   └── api.md
├── contributing/           # Contributor docs
│   ├── index.md
│   └── building-docs.md
└── benchmarks.md           # Performance benchmarks
```

---

## Writing Guidelines

### Page Structure

Each guide should follow this structure:

1. **Title** - Clear feature name
2. **Quick Example** - Minimal working code
3. **Detailed Sections** - Feature breakdown
4. **Real Terminal Output** - Command + output
5. **Tips** - Admonitions with best practices

### Code Examples

Always include runnable examples:

````markdown
```rust
use fast_rich::prelude::*;

fn main() {
    let console = Console::new();
    console.print("[bold]Hello[/]");
}
```
````

### Command + Output Blocks

Show both the command and its output:

````markdown
**Command:**
```bash
cargo run --example hello
```

**Output:**
```
Hello, World!
```
````

### Admonitions

Use admonitions sparingly for important notes:

```markdown
!!! tip "Performance Tip"
    Batch progress updates for better performance.

!!! warning "Breaking Change"
    This API changed in v0.2.0.

!!! note "Feature Flag"
    Requires the `syntax` feature.
```

### Tabs

Use tabs for alternative approaches:

````markdown
=== "Basic"

    ```toml
    [dependencies]
    fast-rich = "0.2.0"
    ```

=== "Full Features"

    ```toml
    [dependencies]
    fast-rich = { version = "0.2.0", features = ["full"] }
    ```
````

---

## Adding a New Guide

1. Create the file in `docs/guides/`:
   ```bash
   touch docs/guides/new-feature.md
   ```

2. Add to navigation in `mkdocs.yml`:
   ```yaml
   nav:
     - Guides:
       - New Feature: guides/new-feature.md
   ```

3. Follow the page structure template above

4. Test locally with `mkdocs serve`

---

## Updating Examples

When library behavior changes:

1. Run the relevant example:
   ```bash
   cargo run --example example_name
   ```

2. Capture the new output

3. Update the documentation to match

4. Verify the example code still compiles:
   ```bash
   cargo build --examples
   ```

---

## MkDocs Configuration

The `mkdocs.yml` file controls the site:

### Theme Settings

```yaml
theme:
  name: material
  palette:
    - scheme: default
      primary: deep purple
      accent: amber
```

### Navigation

```yaml
nav:
  - Home: index.md
  - Getting Started: getting-started.md
  - Guides:
    - guides/index.md
```

### Extensions

```yaml
markdown_extensions:
  - pymdownx.highlight
  - pymdownx.superfences
  - admonition
  - pymdownx.tabbed
```

---

## Deployment

Documentation is automatically deployed via GitHub Actions when changes are pushed to `main`.

Manual deployment:

```bash
mkdocs gh-deploy
```

This builds and pushes to the `gh-pages` branch.

---

## Capturing Terminal Visuals

The documentation includes PNG screenshots for static features and GIF animations for dynamic features.

### Installing the Tools

```bash
# PNG screenshots
go install github.com/homeport/termshot/cmd/termshot@latest

# Professional GIF generation (VHS)
brew install vhs
brew install ttyd
brew install ffmpeg

# SVG output (optional)
npm install -g svg-term-cli
```

### Generating All Visuals

Use the Makefile to regenerate everything:

```bash
make screenshots   # PNG screenshots
make animations    # GIF animations
make all           # Everything + docs
```

### Manual Screenshot (PNG)

For static features like tables, trees, panels:

```bash
~/go/bin/termshot --show-cmd --filename docs/assets/example.png -- \
    cargo run --example example_name
```

### Manual Animation (GIF)

For dynamic features like progress bars, spinners, and live displays, we use [VHS](https://github.com/charmbracelet/vhs) to generate professional GIFs from `.tape` files.

```bash
# Generate a GIF from a tape
vhs docs/assets/progress.tape

# Or use the Makefile to generate all professional animations
make animations
```

### Writing Tape Files

Tape files allow you to simulate a professional terminal session:

```vhs
Output docs/assets/demo.gif
Set Theme "Catppuccin Mocha"
Set FontSize 24
Type "cargo run --example showcase"
Enter
Sleep 5s
```

### When to Use PNG vs GIF

| Content Type | Format | Examples |
|:-------------|:-------|:---------|
| Static output | PNG | Tables, trees, panels, styles |
| Dynamic/animated | GIF | Progress bars, spinners, live displays |
| Hero image | SVG | Homepage hero |

### Embedding in Documentation

PNG:
```markdown
![Description](../assets/screenshot.png)
```

GIF:
```markdown
![Description](../assets/animation.gif)
```

### Code → Run → Visual Format

Always use this format in guides:

```markdown
!!! example "Feature demo"

    **Code**
    
    ```rust
    // example code
    ```

    **Run it**
    
    ```bash
    cargo run --example example_name
    ```

    **What you'll see**

    ![Demo](../assets/demo.png)
```

---

## Style Checklist

Before submitting documentation changes:

- [ ] Code examples compile and run
- [ ] Output matches actual library behavior
- [ ] Consistent formatting with existing pages
- [ ] No broken internal links
- [ ] Spellcheck passed
- [ ] `mkdocs build` completes without warnings
