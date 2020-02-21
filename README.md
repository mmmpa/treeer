# Treeer

This is just a HTML tags builder.

# Usage

```rust
use treeer::prelude::*;
use treeer::helpers::*;

#[test]
fn render_container() {
    let html = Workspace::default()
      .inc(|t| {
          t.div((id("article"), class(&["container"]))).inc(|t| {
              t.h1("Treeer");
              t.p((class(&["description"]), "Treer is just a HTML tag builder"));
          });
      })
      .render_string(100)
      .unwrap();

    assert_eq!(r#"<div class="container" id="article"><h1>Treeer</h1><p class="description">Treeer is just a HTML tag builder</p></div>"#, html)
}
```