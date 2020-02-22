[![CircleCI](https://circleci.com/gh/mmmpa/treeer.svg?style=svg)](https://circleci.com/gh/mmmpa/treeer)

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
          t.div((id("article".into()), class(&["container"]))).inc(|t| {
              t.h1("Treeer");
              t.p((class(&["description"]), "Treer is just a HTML tag builder"));
              t.text("No tag text".into());
          });
      })
      .render_string(100)
      .unwrap();

    assert_eq!(r#"<div class="container" id="article"><h1>Treeer</h1><p class="description">Treeer is just a HTML tag builder</p>No tag text</div>"#, html)
}
```