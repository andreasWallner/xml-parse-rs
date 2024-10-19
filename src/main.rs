use std::marker::PhantomData;

use bumpalo::Bump;

pub enum NodeType {
    Document,
    Element,
    Data,
    CData,
    Comment,
    Declaration,
    Doctype,
    Pi,
}

#[derive(Default)]
pub struct Node<'a> {
    pub name: &'a [u8],
    pub first_attribute: Option<&'a [u8]>,
    pub first_child: Option<&'a [u8]>,
}
impl<'a> Node<'a> {
    pub fn name() -> &'a [u8] {
        todo!()
    }
    pub fn value() -> &'a [u8] {
        todo!()
    }
    pub fn parent() -> &'a Node<'a> {
        todo!()
    }
}

pub struct Error {}

pub struct Parsed<'a> {
    bump: Bump,
    buffer: &'a mut [u8],
}

impl<'a> Parsed<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            bump: Bump::new(),
            buffer,
        }
    }

    pub fn parse(&mut self) -> Result<(), Error> {
        let mut buffer = core::mem::take(&mut self.buffer);
        // parse possible BOM?
        loop {
            if buffer.is_empty() {
                return Ok(());
            }
            // skip whitespace
            if buffer[0] != b'<' {
                return Err(Error {});
            }
            buffer = self.parse_node(&mut buffer[1..]);
        }
    }

    pub fn parse_node<'b>(&mut self, buffer: &'b mut [u8]) -> &'b mut [u8] {
        match buffer[0] {
            _ => self.parse_element(buffer),
        }
    }

    pub fn parse_element<'b>(&mut self, buffer: &'b mut [u8]) -> &'b mut [u8] {
        let (name, buffer) = skip_node_name(buffer);
        let (_, buffer) = skip_space(buffer);

        let node = self.bump.alloc(Node {
            name,
            .. Default::default()
        });

        // TODO parse attributes

        //return buffer;
        let x = match buffer[0] {
            b'>' => self.parse_node_contents(&mut node, &mut buffer[1..]),
            b'/' if buffer[1] == b'>' => &mut buffer[2..],
            _ => todo!(), // error
        };
        x
    }

    pub fn parse_node_contents<'b>(
        &mut self,
        node: &'_ mut Node,
        buffer: &'b mut [u8],
    ) -> &'b mut [u8] {
        // TODO correct whitespace skipping
        let (_, buffer) = skip_space(buffer);
        match buffer[0] {
            b'<' if buffer[1] == b'/' => todo!(),
            b'<' => todo!(),
            _ => todo!(),
        }
    }
}

pub fn skip_space(buffer: &mut [u8]) -> (&mut [u8], &mut [u8]) {
    let pos = buffer
        .iter()
        .position(|c| matches!(c, b' ' | b'\n' | b'\r' | b'\t'))
        .unwrap();
    buffer.split_at_mut(pos)
}

pub fn skip_node_name(buffer: &mut [u8]) -> (&mut [u8], &mut [u8]) {
    let pos = buffer
        .iter()
        .position(|c| matches!(c, 0x00 | b'\n' | b'\r' | b'\t' | b'/' | b'>' | b'?'))
        .unwrap();
    buffer.split_at_mut(pos)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn blub() {
        let input = "<a>content</a>";
        let mut input = input.as_bytes().to_owned();
        let mut parser = Parsed::new(&mut input[..]);
        let _ = parser.parse();
    }
}
