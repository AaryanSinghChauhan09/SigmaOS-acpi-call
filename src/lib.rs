//! SigmaOS-acpi-call: ACPI call model.
//!
//! Real implementation: methods, objects, and validation.
//! Replaces the 2-line stub.

use std::collections::BTreeMap;
use std::fmt;

/// ACPI method.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcpiMethod {
    pub id: String,
    pub name: String,
    pub object: String,
    pub return_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcpiCallError {
    EmptyName,
    DuplicateMethod(String),
}

impl fmt::Display for AcpiCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "method name must not be empty"),
            Self::DuplicateMethod(id) => write!(f, "duplicate method {:?}", id),
        }
    }
}

impl std::error::Error for AcpiCallError {}

/// ACPI call manager.
pub struct AcpiCall {
    pub name: String,
    methods: BTreeMap<String, AcpiMethod>,
}

impl AcpiCall {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            methods: BTreeMap::new(),
        }
    }

    pub fn add_method(&mut self, method: AcpiMethod) -> Result<(), AcpiCallError> {
        if method.name.trim().is_empty() { return Err(AcpiCallError::EmptyName); }
        let id = method.id.clone();
        if self.methods.contains_key(&id) {
            return Err(AcpiCallError::DuplicateMethod(id.clone()));
        }
        self.methods.insert(id, method);
        Ok(())
    }

    pub fn method_count(&self) -> usize { self.methods.len() }

    pub fn get_method(&self, id: &str) -> Option<&AcpiMethod> { self.methods.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_create_aaa() {
        let a = AcpiCall::new("My ACPI Call");
        assert_eq!(a.name, "My ACPI Call");
        assert_eq!(a.method_count(), 0);
    }

    #[test]
    fn test_add_method_aaa() {
        let mut a = AcpiCall::new("My ACPI Call");
        let m = AcpiMethod { id: "m1".into(), name: "_STA".into(), object: "battery".into(), return_type: "int".into() };
        a.add_method(m).unwrap();
        assert_eq!(a.method_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut a = AcpiCall::new("My ACPI Call");
        let m = AcpiMethod { id: "x".into(), name: "".into(), object: "a".into(), return_type: "b".into() };
        assert!(a.add_method(m).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = AcpiCall::new("My ACPI Call");
        let m = AcpiMethod { id: "x".into(), name: "X".into(), object: "a".into(), return_type: "b".into() };
        a.add_method(m).unwrap();
        let m2 = AcpiMethod { id: "x".into(), name: "X".into(), object: "a".into(), return_type: "b".into() };
        assert!(a.add_method(m2).is_err());
    }
}
