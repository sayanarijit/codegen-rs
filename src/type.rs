use std::fmt::{self, Write};

use crate::formatter::Formatter;

/// Defines a type.
#[derive(Debug, Clone)]
pub struct Type {
    name: String,
    generics: Vec<Type>,
    visibility: Option<String>,
}

impl Type {
    /// Return a new type with the given name.
    pub fn new(name: &str) -> Self {
        Type {
            name: name.to_string(),
            generics: vec![],
            visibility: None,
        }
    }

    /// Add a generic to the type.
    pub fn generic<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        // Make sure that the name doesn't already include generics
        assert!(
            !self.name.contains("<"),
            "type name already includes generics"
        );

        self.generics.push(ty.into());
        self
    }

    /// Change the visibility of the type.
    pub fn vis(&mut self, visibility: &str) -> &mut Self {
        self.visibility = Some(visibility.to_string());
        self
    }

    /// Formats the struct using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if let Some(vis) = self.visibility.as_ref() {
            write!(fmt, "{} ", vis)?;
        };

        write!(fmt, "{}", self.name)?;
        Type::fmt_slice(&self.generics, fmt)
    }

    fn fmt_slice(generics: &[Type], fmt: &mut Formatter<'_>) -> fmt::Result {
        if !generics.is_empty() {
            write!(fmt, "<")?;

            for (i, ty) in generics.iter().enumerate() {
                if i != 0 {
                    write!(fmt, ", ")?
                }
                ty.fmt(fmt)?;
            }

            write!(fmt, ">")?;
        }

        Ok(())
    }
}

impl<'a> From<&'a str> for Type {
    fn from(src: &'a str) -> Self {
        Type::new(src)
    }
}

impl From<String> for Type {
    fn from(src: String) -> Self {
        Self::new(&src)
    }
}

impl<'a> From<&'a String> for Type {
    fn from(src: &'a String) -> Self {
        Type::new(src)
    }
}

impl<'a> From<&'a Type> for Type {
    fn from(src: &'a Type) -> Self {
        src.clone()
    }
}
