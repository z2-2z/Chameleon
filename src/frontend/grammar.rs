use std::collections::BTreeMap;

use crate::frontend::range::SourceRange;

pub type NumbersetId = usize;
pub type ContainerId = usize;
pub type StringId = usize;

pub enum Endianness {
    Big,
    Little,
    Native,
}

pub enum Scheduling {
    RoundRobin,
    Random,
}

pub enum GrammarOption {
    Endianness(Endianness),
    Scheduling(Scheduling),
    // Inline ?
}

pub enum VariableOption {
    Optional,
    Repeats(NumbersetId),
}

pub enum IntegerValue {
    FromSet(NumbersetId),
    Any,
}

pub enum BytearrayValue {
    Any(NumbersetId),
    Literal(StringId),
}

pub enum VariableType {
    U8(IntegerValue),
    I8(IntegerValue),
    U16(IntegerValue),
    I16(IntegerValue),
    U32(IntegerValue),
    I32(IntegerValue),
    U64(IntegerValue),
    I64(IntegerValue),
    String(BytearrayValue),
    Bytes(BytearrayValue),
    Oneof(ContainerId),
    ContainerRef(ContainerId),
}

pub struct Variable {
    options: Vec<VariableOption>,
    typ: VariableType,
}

pub struct Container {
    id: ContainerId,
    name: Option<SourceRange>,
    local_options: Vec<GrammarOption>,
    variables: Vec<Variable>,
}

pub struct Numberset<T> {
    id: NumbersetId,
    content: Vec<std::ops::Range<T>>,
}

pub enum NumbersetType {
    U8(Numberset<u8>),
    U16(Numberset<u16>),
    U32(Numberset<u32>),
    U64(Numberset<u64>),
}

pub struct Grammar {
    global_options: Vec<GrammarOption>,
    containers: BTreeMap<ContainerId, Container>,
    number_sets: BTreeMap<NumbersetId, NumbersetType>,
    strings: BTreeMap<StringId, SourceRange>,
    root: Option<ContainerId>,
}
impl Grammar {
    pub fn new() -> Self {
        Self {
            global_options: Vec::new(),
            containers: BTreeMap::new(),
            number_sets: BTreeMap::new(),
            strings: BTreeMap::new(),
            root: None,
        }
    }
    
    pub fn add_global_option(&mut self, option: GrammarOption) {
        
    }
}
