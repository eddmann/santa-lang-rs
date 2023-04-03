use crate::evaluator::Object;
use crate::parser::ast::Statement;
use std::cell::RefCell;
use std::rc::Rc;

pub type EnvironmentRef = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
pub struct Environment {
    store: Vec<(String, Rc<Object>, bool)>,
    sections: Vec<(String, Rc<Statement>)>,
    outer: Option<EnvironmentRef>,
}

pub struct EnvironmentErr {
    pub message: String,
}

impl Environment {
    pub fn new() -> EnvironmentRef {
        Rc::new(RefCell::new(Environment {
            store: vec![],
            sections: vec![],
            outer: None,
        }))
    }

    pub fn from(outer: EnvironmentRef) -> EnvironmentRef {
        Rc::new(RefCell::new(Environment {
            store: vec![],
            sections: vec![],
            outer: Some(outer),
        }))
    }

    pub fn get_sections(&self, name: &str) -> Vec<Rc<Statement>> {
        self.sections
            .iter()
            .filter_map(|(name_, body)| if name_ == name { Some(Rc::clone(body)) } else { None })
            .collect()
    }

    pub fn add_section(&mut self, name: &str, body: Rc<Statement>) {
        self.sections.push((name.to_owned(), body))
    }

    pub fn declare_variable(&mut self, name: &str, value: Rc<Object>, mutable: bool) -> Result<(), EnvironmentErr> {
        for (name_, _, _) in &self.store {
            if name_ == name {
                return Err(EnvironmentErr {
                    message: format!("Variable '{}' has already been declared", name),
                });
            }
        }

        self.store.push((name.to_owned(), value, mutable));
        Ok(())
    }

    pub fn get_variable(&self, name: &str) -> Option<Rc<Object>> {
        for (name_, value, _) in &self.store {
            if name_ == name {
                return Some(Rc::clone(value));
            }
        }

        if let Some(outer) = &self.outer {
            return outer.borrow().get_variable(name);
        }

        None
    }

    pub fn assign_variable(&mut self, name: &str, value: Rc<Object>) -> Result<(), EnvironmentErr> {
        for (name_, value_, mutable) in self.store.iter_mut() {
            if *name_ == name {
                if !*mutable {
                    return Err(EnvironmentErr {
                        message: format!("Variable '{}' is not mutable", name),
                    });
                }

                *value_ = value;
                return Ok(());
            }
        }

        if let Some(outer) = &self.outer {
            return outer.borrow_mut().assign_variable(name, value);
        }

        Err(EnvironmentErr {
            message: format!("Variable '{}' has not been declared", name),
        })
    }

    pub fn set_variable(&mut self, name: &str, value: Rc<Object>) {
        for (name_, value_, _) in self.store.iter_mut() {
            if *name_ == name {
                *value_ = value;
                return;
            }
        }

        self.store.push((name.to_owned(), value, false));
    }
}
