use std::io::Write;
use crate::treeer::errors::TreeerErr;

pub trait Render {
    fn render_string(&self, cap: usize) -> Result<String, TreeerErr> {
        let mut s = Vec::with_capacity(cap);
        self.write(&mut s)?;
        Ok(String::from_utf8(s).unwrap())
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), TreeerErr>;
}
