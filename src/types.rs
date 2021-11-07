/*! 
This module contains types which implement the [`Tokens`] interface. You
won't often need to import this module unless you wish to explicitly name
the types in quesiton. 

Prefer to remain generic where possible, for example by using 
`t: impl Tokens<char>` over `t: StrTokens<'a>` as an argument.
*/
use super::{ IntoTokens, Tokens };

/// This is what we are given back if we call `into_tokens()` on
/// a `&[T]`. It implements the [`Tokens`] interface.
pub struct SliceTokens<'a, Item> {
    slice: &'a [Item],
    cursor: usize,
}

pub struct SliceTokensCheckpoint(usize);

impl <'a, Item> SliceTokens<'a, Item> {
    /// Return the parsed portion of the slice.
    pub fn consumed(&self) -> &'a [Item] {
        &self.slice[..self.cursor]
    }

    /// Return the unparsed remainder of the slice.
    pub fn remaining(&self) -> &'a [Item] {
        &self.slice[self.cursor..]
    }
}

impl <'a, Item> From<SliceTokens<'a, Item>> for &'a [Item] {
    fn from(toks: SliceTokens<'a, Item>) -> Self {
        toks.slice
    }
}

impl <'a, Item> Iterator for SliceTokens<'a, Item> {
    type Item = &'a Item;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.slice.get(self.cursor);
        self.cursor += 1;
        res
    }
}

impl <'a, Item> Tokens for SliceTokens<'a, Item> {
    type CheckPoint = SliceTokensCheckpoint;

    fn save_checkpoint(&self) -> Self::CheckPoint {
        SliceTokensCheckpoint(self.cursor)
    }
    fn rewind_to_checkpoint(&mut self, checkpoint: Self::CheckPoint) {
        self.cursor = checkpoint.0;
    }
}

impl <'a, Item> IntoTokens<&'a Item> for SliceTokens<'a, Item> {
    type Tokens = Self;
    fn into_tokens(self) -> Self {
        self
    }
}

impl <'a, Item> IntoTokens<&'a Item> for &'a [Item] {
    type Tokens = SliceTokens<'a, Item>;
    fn into_tokens(self: Self) -> Self::Tokens {
        SliceTokens {
            slice: self,
            cursor: 0,
        }
    }
}

/// This is what we are given back if we call `into_tokens()` on
/// a `&str`. It implements the [`Tokens`] interface.
pub struct StrTokens<'a> {
    str: &'a str,
    cursor: usize
}

pub struct StrTokensCheckpoint(usize);

impl <'a> StrTokens<'a> {
    /// Return the parsed portion of the str.
    pub fn consumed(&self) -> &'a str {
        &self.str[..self.cursor]
    }

    /// Return the unparsed remainder of the str.
    pub fn remaining(&self) -> &'a str {
        &self.str[self.cursor..]
    }
}

impl <'a> From<StrTokens<'a>> for &'a str {
    fn from(toks: StrTokens<'a>) -> Self {
        toks.str
    }
}

impl <'a> Iterator for StrTokens<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor == self.str.len() {
            return None;
        }

        // Cursor should always start at a valid char boundary.
        // So, we just find the next char boundary and return the
        // char between those two.
        let mut next_char_boundary = self.cursor + 1;
        while !self.str.is_char_boundary( next_char_boundary) {
            next_char_boundary += 1;
        }

        // We have to go to &str and then char. Unchecked because we know
        // that we are on a valid boundary. There's probably a quicker way..
        let next_char = unsafe {
            self.str.get_unchecked(self.cursor..next_char_boundary)
        }.chars().next().unwrap();

        self.cursor = next_char_boundary;
        Some(next_char)
    }
}

impl <'a> Tokens for StrTokens<'a> {
    type CheckPoint = StrTokensCheckpoint;

    fn save_checkpoint(&self) -> Self::CheckPoint {
        StrTokensCheckpoint(self.cursor)
    }
    fn rewind_to_checkpoint(&mut self, checkpoint: Self::CheckPoint) {
        self.cursor = checkpoint.0;
    }
}

impl <'a> IntoTokens<char> for StrTokens<'a> {
    type Tokens = Self;
    fn into_tokens(self) -> Self {
        self
    }
}

impl <'a> IntoTokens<char> for &'a str {
    type Tokens = StrTokens<'a>;
    fn into_tokens(self: Self) -> Self::Tokens {
        StrTokens {
            str: self,
            cursor: 0,
        }
    }
}