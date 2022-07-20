use std::collections::BTreeMap;
use std::collections::btree_map::Values;
use std::default::Default;
use std::ops::Range;

use crate::frontend::SourceRange;

pub type NumbersetId = usize;
pub type ContainerId = usize;
pub type StringId = usize;

/// Allowed values for the endianness option in containers
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
    pub fn new(id: ContainerId, name: Option<SourceRange>) -> Self {
        Self {
            id,
            options: ContainerOptions::default(),
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
pub struct Numberset<T> {
    id: NumbersetId,
    content: Vec<Range<T>>,
}

/// Lists the different types numbersets can have
pub enum NumbersetType {
    U8(Numberset<u8>),
    U16(Numberset<u16>),
    U32(Numberset<u32>),
    U64(Numberset<u64>),
}

/// Represents an entire grammar
pub struct Grammar {
    options: ContainerOptions,
    containers: BTreeMap<ContainerId, Container>,
    container_cursor: ContainerId,
    number_sets: BTreeMap<NumbersetId, NumbersetType>,
    strings: BTreeMap<StringId, SourceRange>,
    root: Option<ContainerId>,
}
impl Grammar {
    pub fn new() -> Self {
        Self {
            options: ContainerOptions::default(),
            containers: BTreeMap::new(),
            container_cursor: ContainerId::default(),
            number_sets: BTreeMap::new(),
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
    
    pub fn containers(&self) -> Values<'_, ContainerId, Container> {
        self.containers.values()
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
