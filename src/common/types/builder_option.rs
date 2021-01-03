pub trait BuilderOption {}
pub trait BuilderOptionDefined: BuilderOption {}
pub trait BuilderOptionUndefined: BuilderOption {}

#[derive(Copy, Clone, Default)]
pub struct Defined;
#[derive(Copy, Clone, Default)]
pub struct Undefined;

impl BuilderOption for Defined {}
impl BuilderOption for Undefined {}
impl BuilderOptionDefined for Defined {}
impl BuilderOptionUndefined for Undefined {}
