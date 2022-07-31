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

#[derive(PartialEq, Copy, Clone)]
pub enum ContainerType {
    Oneof,
    Struct,
}

/// A container for variables
pub struct Container {
    id: ContainerId,
    typ: ContainerType,
    options: ContainerOptions,
    name: Option<SourceRange>,
    variables: Vec<Variable>,
}
impl Container {
    pub fn new(id: ContainerId, typ: ContainerType, options: ContainerOptions, name: Option<SourceRange>) -> Self {
        Self {
            id,
            typ,
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
    
    pub fn typ(&self) -> ContainerType {
        self.typ
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
                    if self.containers.get(&id).unwrap().typ() == ContainerType::Struct {
                        ret.push(id.clone());
                    }
                },
                VariableType::Oneof(id) => {
                    let mut callees = self.container_callees(*id);
                    ret.append(&mut callees);
                },
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
    
    pub fn num_paths(&self, id: ContainerId) -> usize {
        let mut num_paths: usize = 1;
        
        for var in &self.containers.get(&id).unwrap().variables {
            match &var.typ {
                VariableType::Oneof(id) => {
                    num_paths = num_paths.saturating_mul(self.num_paths_oneof(*id));
                },
                VariableType::ContainerRef(id) => {
                    num_paths = num_paths.saturating_mul(self.num_paths(*id));
                },
                _ => {},
            }
        }
        
        num_paths
    }
    
    fn num_paths_oneof(&self, id: ContainerId) -> usize {
        let mut num_paths: usize = 0;
        
        for var in &self.containers.get(&id).unwrap().variables {
            match &var.typ {
                VariableType::Oneof(id) => {
                    num_paths = num_paths.saturating_add(self.num_paths_oneof(*id));
                },
                VariableType::ContainerRef(id) => {
                    num_paths = num_paths.saturating_add(self.num_paths(*id));
                },
                _ => {
                    num_paths += 1;
                },
            }
        }
        
        num_paths
    }
    
    pub fn container_size(&self, id: ContainerId, calc_max: bool) -> usize {
        let mut size: usize = 0;
        
        for var in &self.containers.get(&id).unwrap().variables {
            size = size.saturating_add(self.variable_size(var, calc_max));
        }
        
        size
    }
    
    fn container_size_bound(&self, id: ContainerId, calc_max: bool) -> usize {
        let mut size: usize = if calc_max {
            0
        } else {
            usize::MAX
        };
        
        for var in &self.containers.get(&id).unwrap().variables {
            if calc_max {
                size = std::cmp::max(size, self.variable_size(var, calc_max));
            } else {
                size = std::cmp::min(size, self.variable_size(var, calc_max));
            }
        }
        
        size
    }
    
    fn variable_size(&self, var: &Variable, calc_max: bool) -> usize {
        if !calc_max && var.options.optional {
            return 0;
        }
        
        let mut var_size = match &var.typ {
            VariableType::Oneof(id) => self.container_size_bound(*id, calc_max),
            VariableType::ContainerRef(id) => self.container_size(*id, calc_max),
            VariableType::U8(_) |
            VariableType::I8(_) => 1,
            VariableType::U16(_) |
            VariableType::I16(_) => 2,
            VariableType::U32(_) |
            VariableType::I32(_) => 4,
            VariableType::U64(_) |
            VariableType::I64(_) => 8,
            VariableType::Bytes(bytearray) |
            VariableType::String(bytearray) => {
                match bytearray {
                    BytearrayValue::Any(id) => {
                        self.get_numberset_bound(*id, calc_max)
                    },
                    BytearrayValue::Literal(id) => {
                        self.strings.get(id).unwrap().len()
                    },
                }
            },
            VariableType::ResolveContainerRef(_) => panic!("Encountered container ref after parsing stage"),
        };
        
        if let Some(id) = &var.options.repeats {
            var_size = var_size.saturating_mul(self.get_numberset_bound(*id, calc_max));
        }
        
        var_size
    }
    
    fn get_numberset_bound(&self, id: NumbersetId, calc_max: bool) -> usize {
        let mut ret: usize = if calc_max {
            0
        } else {
            usize::MAX
        };
        
        match self.numbersets.get(&id).unwrap() {
            NumbersetType::U32(v) => {
                for range in v {
                    if calc_max {
                        ret = std::cmp::max(ret, range.end as usize);
                    } else {
                        ret = std::cmp::min(ret, range.start as usize);
                    }
                }
            },
            _ => panic!("Tried to get maximum of non-u32 numberset"),
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
