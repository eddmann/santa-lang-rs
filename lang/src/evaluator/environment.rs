use crate::evaluator::Object;
use crate::parser::ast::Section;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type EnvironmentRef = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, (Rc<Object>, bool)>,
    sections: Vec<(String, Rc<Section>)>,
    outer: Option<EnvironmentRef>,
}

pub struct EnvironmentErr {
    pub message: String,
}

impl Environment {
    pub fn new() -> EnvironmentRef {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            sections: vec![],
            outer: None,
        }))
    }

    pub fn from(outer: EnvironmentRef) -> EnvironmentRef {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            sections: vec![],
            outer: Some(outer),
        }))
    }

    pub fn get_sections(&self, name: &str) -> Vec<Rc<Section>> {
        self.sections
            .iter()
            .filter_map(|(name_, body)| if name_ == name { Some(Rc::clone(body)) } else { None })
            .collect()
    }

    pub fn add_section(&mut self, name: &str, body: Rc<Section>) {
        self.sections.push((name.to_owned(), body))
    }

    pub fn declare_variable(&mut self, name: &str, value: Rc<Object>, mutable: bool) -> Result<(), EnvironmentErr> {
        if self.store.contains_key(name) {
            return Err(EnvironmentErr {
                message: format!("Variable '{}' has already been declared", name),
            });
        }

        self.store.insert(name.to_owned(), (value, mutable));
        Ok(())
    }

    pub fn get_variable(&self, name: &str) -> Option<Rc<Object>> {
        if let Some((value, _)) = self.store.get(name) {
            return Some(Rc::clone(value));
        }

        if let Some(outer) = &self.outer {
            return outer.borrow().get_variable(name);
        }

        None
    }

    pub fn assign_variable(&mut self, name: &str, value: Rc<Object>) -> Result<(), EnvironmentErr> {
        if let Some((stored_value, mutable)) = self.store.get_mut(name) {
            if !*mutable {
                return Err(EnvironmentErr {
                    message: format!("Variable '{}' is not mutable", name),
                });
            }

            *stored_value = value;
            return Ok(());
        }

        if let Some(outer) = &self.outer {
            return outer.borrow_mut().assign_variable(name, value);
        }

        Err(EnvironmentErr {
            message: format!("Variable '{}' has not been declared", name),
        })
    }

    pub fn set_variable(&mut self, name: &str, value: Rc<Object>) {
        if let Some((stored_value, _)) = self.store.get_mut(name) {
            *stored_value = value;
            return;
        }

        self.store.insert(name.to_owned(), (value, false));
    }
}
