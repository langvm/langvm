// Copyright 2024 Jelly Terra
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::ir::*;

pub struct BasicBlock {
    pub Operations: Vec<Operation>,

    pub Precedences: Vec<isize>,
    pub Successors: Vec<isize>,
}

pub struct Function {
    pub BasicBlocks: Vec<BasicBlock>,

    pub Params: Vec<Type>,
    pub Result: Type,
}

pub enum Operation {
    Invoke(Invoke),
    Goto(Goto),
}

#[derive(Default)]
pub struct Invoke {}

#[derive(Default)]
pub struct Goto {}
