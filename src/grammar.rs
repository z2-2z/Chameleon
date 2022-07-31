use std::collections::BTreeMap;
use std::collections::btree_map::{Values, Keys};
use std::default::Default;
use std::ops::Range;
use ahash;
use std::hash::Hasher;

use crate::frontend::SourceRange;

pub type NumbersetId = usize;
pub type ContainerId = usize;
pub type StringId = usize;

/// Allowed values for the endianness option in containers
#[derive(Clone)]
pub enum Endianness {
    Big,
    Little,
    Native,
}
impl Default for Endianness {
    fn default() -> Self {
        Endianness::Native
    }
}

/// Allowed values for the scheduling option in containers
#[derive(Clone)]
pub enum Scheduling {
    RoundRobin,
    Random,
}
impl Default for Scheduling {
    fn default() -> Self {
        Scheduling::RoundRobin
    }
}

/// Storage for all possible options in a container
#[derive(Clone)]
pub struct ContainerOptions {
    endianness: Endianness,
    scheduling: Scheduling,
}
impl Default for ContainerOptions {
    fn default() -> Self {
        Self {
            endianness: Endianness::default(),
            scheduling: Scheduling::default(),
        }
    }
}
impl ContainerOptions {
    pub fn set_endianness(&mut self, value: Endianness) {
        self.endianness = value;
    }
    
    pub fn set_scheduling(&mut self, value: Scheduling) {
        self.scheduling = value;
    }
}

pub trait HasOptions {
    fn options_mut(&mut self) -> &mut ContainerOptions;
    fn options(&self) -> &ContainerOptions;
}

/// Storage for all possible options of a variable
pub struct VariableOptions {
    optional: bool,
    repeats: Option<NumbersetId>,
}
impl Default for VariableOptions {
    fn default() -> Self {
        Self {
            optional: false,
            repeats: None,
        }
    }
}
impl VariableOptions {
    pub fn set_optional(&mut self) {
        self.optional = true;
    }
    
    pub fn set_repeats(&mut self, numberset: NumbersetId) {
        self.repeats = Some(numberset);
    }
}

/// Possible values for an integer
pub enum IntegerValue {
    FromSet(NumbersetId),
    Any,
}

/// Possible values for a string or bytes type
pub enum BytearrayValue {
    Any(NumbersetId),
    Literal(StringId),
}

/// All possible types for variables
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
    ResolveContainerRef(SourceRange),
}

/// A single variable in a container
pub struct Variable {
    options: VariableOptions,
    typ: VariableType,
}
impl Variable {
    pub fn new(options: VariableOptions, typ: VariableType) -> Self {
        Self {
            options,
            typ
        }
    }
}

/// A container for variables
pub struct Container {
    id: ContainerId,
    options: ContainerOptions,
    name: Option<SourceRange>,
    variables: Vec<Variable>,
}
impl Container {
    pub fn new(id: ContainerId, options: ContainerOptions, name: Option<SourceRange>) -> Self {
        Self {
            id,
            options,
            name,
            variables: Vec::new(),
        }
    }
    
    pub fn name(&self) -> Option<&SourceRange> {
        self.name.as_ref()
    }
    
    pub fn id(&self) -> ContainerId {
        self.id
    }
    
    pub fn add_variable(&mut self, var: Variable) {
        self.variables.push(var);
    }
    
    pub fn resolve_reference(&mut self, var: usize, target: ContainerId) {
        self.variables[var].typ = VariableType::ContainerRef(target);
    }
    
    pub fn variables(&self) -> &[Variable] {
        &self.variables
    }
}
impl HasOptions for Container {
    fn options_mut(&mut self) -> &mut ContainerOptions {
        &mut self.options
    }
    
    fn options(&self) -> &ContainerOptions {
        &self.options
    }
}

/// A set of numbers of generic type
pub type Numberset<T> = Vec<Range<T>>;

/// Lists the different types numbersets can have
pub enum NumbersetType {
    U8(Numberset<u8>),
    I8(Numberset<i8>),
    U16(Numberset<u16>),
    I16(Numberset<i16>),
    U32(Numberset<u32>),
    I32(Numberset<i32>),
    U64(Numberset<u64>),
    I64(Numberset<i64>),
}
impl NumbersetType {
    fn id(&self) -> NumbersetId {
        let mut hasher = ahash::AHasher::new_with_keys(0, 0);
        
        match self {
            NumbersetType::U8(set) => {
                hasher.write("U8".as_bytes());
                for range in set {
                    hasher.write_u8(range.start);
                    hasher.write_u8(range.end);
                }
            },
            NumbersetType::I8(set) => {
                hasher.write("I8".as_bytes());
                for range in set {
                    hasher.write_i8(range.start);
                    hasher.write_i8(range.end);
                }
            },
            NumbersetType::U16(set) => {
                hasher.write("U16".as_bytes());
                for range in set {
                    hasher.write_u16(range.start);
                    hasher.write_u16(range.end);
                }
            },
            NumbersetType::I16(set) => {
                hasher.write("I16".as_bytes());
                for range in set {
                    hasher.write_i16(range.start);
                    hasher.write_i16(range.end);
                }
            },
            NumbersetType::U32(set) => {
                hasher.write("U32".as_bytes());
                for range in set {
                    hasher.write_u32(range.start);
                    hasher.write_u32(range.end);
                }
            },
            NumbersetType::I32(set) => {
                hasher.write("I32".as_bytes());
                for range in set {
                    hasher.write_i32(range.start);
                    hasher.write_i32(range.end);
                }
            },
            NumbersetType::U64(set) => {
                hasher.write("U64".as_bytes());
                for range in set {
                    hasher.write_u64(range.start);
                    hasher.write_u64(range.end);
                }
            },
            NumbersetType::I64(set) => {
                hasher.write("I64".as_bytes());
                for range in set {
                    hasher.write_i64(range.start);
                    hasher.write_i64(range.end);
                }
            },
        }
        
        hasher.finish() as NumbersetId
    }
}

/// Represents an entire grammar
pub struct Grammar {
    options: ContainerOptions,
    containers: BTreeMap<ContainerId, Container>,
    container_cursor: ContainerId,
    numbersets: BTreeMap<NumbersetId, NumbersetType>,
    strings: BTreeMap<StringId, Vec<u8>>,
    root: Option<ContainerId>,
}
impl Grammar {
    pub fn new() -> Self {
        Self {
            options: ContainerOptions::default(),
            containers: BTreeMap::new(),
            container_cursor: ContainerId::default(),
            numbersets: BTreeMap::new(),
            strings: BTreeMap::new(),
            root: None,
        }
    }
    
    pub fn add_container(&mut self, container: Container) {
        let key = container.id;
        
        assert!( self.containers.insert(key, container).is_none() );
    }
    
    pub fn reserve_container_id(&mut self) -> ContainerId {
        let ret = self.container_cursor;
        self.container_cursor += 1;
        ret
    }
    
    pub fn set_root(&mut self, root: ContainerId) {
        self.root = Some(root);
    }
    
    pub fn root(&self) -> Option<&ContainerId> {
        (&self.root).as_ref()
    }
    
    pub fn container(&self, id: ContainerId) -> Option<&Container> {
        self.containers.get(&id)
    }
    
    pub fn container_mut(&mut self, id: ContainerId) -> Option<&mut Container> {
        self.containers.get_mut(&id)
    }
    
    pub fn containers(&self) -> Values<'_, ContainerId, Container> {
        self.containers.values()
    }
    
    pub fn container_ids(&self) -> Keys<'_, ContainerId, Container> {
        self.containers.keys()
    }
    
    pub fn add_numberset(&mut self, set: NumbersetType) -> NumbersetId {
        let id = set.id();
        
        if self.numbersets.contains_key(&id) {
            id
        } else {
            assert!( self.numbersets.insert(id, set).is_none() );
            id
        }
    }
    
    pub fn add_string(&mut self, buf: Vec<u8>) -> StringId {
        let mut hasher = ahash::AHasher::new_with_keys(0, 0);
        hasher.write(&buf);
        let id = hasher.finish() as StringId;
        
        if self.strings.contains_key(&id) {
            id
        } else {
            assert!( self.strings.insert(id, buf).is_none() );
            id
        }
    }
    
    pub fn container_callees(&self, id: ContainerId) -> Vec<ContainerId> {
        let mut ret = Vec::<ContainerId>::new();
        
        for var in &self.containers.get(&id).unwrap().variables {
            match &var.typ {
                VariableType::ContainerRef(id) => {
                    ret.push(id.clone());
                },
                VariableType::Oneof(id) => {
                    let mut callees = self.container_callees(*id);
                    ret.append(&mut callees);
                },
                VariableType::ResolveContainerRef(_) => panic!("Encountered unresolved reference after parsing stage"),
                _ => {},
            }
        }
        
        ret
    }
    
    pub fn unresolved_names(&self) -> Vec<(ContainerId, usize, SourceRange)> {
        let mut ret = Vec::new();
        
        for (container_id, container) in self.containers.iter() {
            for i in 0..container.variables.len() {
                match &container.variables[i].typ {
                    VariableType::ResolveContainerRef(range) => {
                        ret.push( (*container_id, i, range.clone()) );
                    },
                    _ => {},
                }
            }
        }
        
        ret
    }
}

impl HasOptions for Grammar {
    fn options_mut(&mut self) -> &mut ContainerOptions {
        &mut self.options
    }
    
    fn options(&self) -> &ContainerOptions {
        &self.options
    }
}
