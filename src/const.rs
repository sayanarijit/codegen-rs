use std::fmt::{self, Write};

use crate::field::Field;
use crate::fields::Fields;
use crate::formatter::Formatter;
use crate::type_def::TypeDef;

use crate::r#type::Type;

/// Defines a struct.
#[derive(Debug, Clone)]
pub struct Const {
    type_def: TypeDef,
    ty: Type,

    /// Const fields
    value: String,
}

impl Const {
    /// Return a structure definition with the provided name
    pub fn new<T>(name: &str, ty: T, value: &str) -> Self
    where
        T: Into<Type>,
    {
        Const {
            type_def: TypeDef::new(name),
            ty: ty.into(),
            value: value.to_string(),
        }
    }

    /// Returns a reference to the type
    pub fn ty(&self) -> &Type {
        &self.ty
    }

    /// Set the structure visibility.
    pub fn vis(&mut self, vis: &str) -> &mut Self {
        self.type_def.vis(vis);
        self
    }

    /// Add a generic to the const.
    pub fn generic(&mut self, name: &str) -> &mut Self {
        self.ty.generic(name);
        self
    }

    /// Add a `where` bound to the const.
    pub fn bound<T>(&mut self, name: &str, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        self.type_def.bound(name, ty);
        self
    }

    /// Set the structure documentation.
    pub fn doc(&mut self, docs: &str) -> &mut Self {
        self.type_def.doc(docs);
        self
    }

    /// Add a new type that the const should derive.
    pub fn derive(&mut self, name: &str) -> &mut Self {
        self.type_def.derive(name);
        self
    }

    /// Specify lint attribute to supress a warning or error.
    pub fn allow(&mut self, allow: &str) -> &mut Self {
        self.type_def.allow(allow);
        self
    }

    /// Specify representation.
    pub fn repr(&mut self, repr: &str) -> &mut Self {
        self.type_def.repr(repr);
        self
    }

    /// Formats the const using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        self.type_def.fmt_head("const", &[self.ty.clone()], fmt)?;

        write!(fmt, " = {};\n", self.value)?;

        Ok(())
    }
}
