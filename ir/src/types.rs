// Copyright 2024 LangVM Project
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

pub struct Field {}

pub enum PredefinedType {
    U8,
    U16,
    U32,
    U64,
    
    I8,
    I16,
    I32,
    I64,
}

pub enum Type {
    None,
    Predefined(PredefinedType),
    Struct(Box<StructType>),
    Trait(Box<TraitType>),
    Func(Box<FuncType>),
}

pub struct StructType {
    pub Fields: Field,
}

pub struct TraitType {
    pub Functions: Vec<Field>,
}

pub struct FuncType {
    pub Params: Vec<Field>,
    pub Result: Type,
}
