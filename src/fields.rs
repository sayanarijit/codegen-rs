use std::fmt::{self, Write};

use crate::field::Field;
use crate::formatter::Formatter;

use crate::r#type::Type;

/// Defines a set of fields.
#[derive(Debug, Clone)]
pub enum Fields {
    Empty,
    Tuple(Vec<Type>),
    Named(Vec<Field>),
}

impl Fields {
    pub fn push_named(&mut self, field: Field) -> &mut Self {
        match *self {
            Fields::Empty => {
                *self = Fields::Named(vec![field]);
            }
            Fields::Named(ref mut fields) => {
                fields.push(field);
            }
            _ => panic!("field list is named"),
        }

        self
    }

    pub fn push_tuple<T>(&mut self, ty: T) -> &mut Self
    where
        T: Into<Type>,
    {
        let ty = ty.into();
        match *self {
            Fields::Empty => {
                *self = Fields::Tuple(vec![ty]);
            }
            Fields::Tuple(ref mut types) => {
                types.push(ty);
            }
            _ => panic!("field list is tuple"),
        }

        self
    }

    pub fn last_tuple_field_mut(&mut self) -> Option<&mut Type> {
        match self {
            Self::Tuple(fields) => fields.last_mut(),
            _ => panic!("field list is tuple"),
        }
    }

    pub fn last_named_field_mut(&mut self) -> Option<&mut Field> {
        match self {
            Self::Named(fields) => fields.last_mut(),
            _ => panic!("field list is tuple"),
        }
    }

    pub fn field<T>(&mut self, name: &str, ty: T) -> &mut Field
    where
        T: Into<Type>,
    {
        self.push_named(Field::new(name, ty));
        self.last_named_field_mut().unwrap()
    }

    pub fn tuple_field<T>(&mut self, ty: T) -> &mut Type
    where
        T: Into<Type>,
    {
        self.push_tuple(ty);
        self.last_tuple_field_mut().unwrap()
    }

    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Fields::Named(ref fields) => {
                assert!(!fields.is_empty());

                fmt.block(|fmt| {
                    for f in fields {
                        if !f.documentation.is_empty() {
                            for doc in &f.documentation {
                                write!(fmt, "/// {}\n", doc)?;
                            }
                        }
                        if !f.annotation.is_empty() {
                            for ann in &f.annotation {
                                write!(fmt, "{}\n", ann)?;
                            }
                        }
                        if let Some(visibility) = &f.visibility {
                            write!(fmt, "{} ", visibility)?;
                        }
                        write!(fmt, "{}: ", f.name)?;
                        f.ty.fmt(fmt)?;
                        write!(fmt, ",\n")?;
                    }

                    Ok(())
                })?;
            }
            Fields::Tuple(ref tys) => {
                assert!(!tys.is_empty());

                write!(fmt, "(")?;

                for (i, ty) in tys.iter().enumerate() {
                    if i != 0 {
                        write!(fmt, ", ")?;
                    }
                    ty.fmt(fmt)?;
                }

                write!(fmt, ")")?;
            }
            Fields::Empty => {}
        }

        Ok(())
    }
}
