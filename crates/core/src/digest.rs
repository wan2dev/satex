use std::borrow::Cow;

pub trait Digester<M> {
    fn digest<'a>(&self, input: &'a M) -> Cow<'a, [u8]>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultDigester;

impl<M> Digester<M> for DefaultDigester {
    fn digest<'b>(&self, _: &'b M) -> Cow<'b, [u8]> {
        const BYTES: &[u8; 0] = &[];
        Cow::from(BYTES)
    }
}
