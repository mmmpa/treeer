mod treeer;

pub use crate::treeer::prelude;
pub use crate::treeer::helpers;

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::helpers::*;

    #[test]
    fn render_container() {
        let html = Workspace::default()
          .inc(|t| {
              t.div(()).inc(|t| {
                  t.p(());
              });
          })
          .render_string(100)
          .unwrap();

        assert_eq!("<div><p></p></div>", html)
    }

    #[test]
    fn render_self_contained() {
        let html = Workspace::default()
          .inc(|t| { t.img(()); })
          .render_string(100)
          .unwrap();

        assert_eq!("<img />", html)
    }


    #[test]
    fn render_attr() {
        let html = Workspace::default()
          .inc(|t| {
              t.div((id("container"), disabled())).inc(|t| {
                  t.img((src("/example.com"), disabled()));
              });
          })
          .render_string(100)
          .unwrap();

        assert_eq!(r#"<div id="container" disabled><img src="/example.com" disabled /></div>"#, html)
    }
}
