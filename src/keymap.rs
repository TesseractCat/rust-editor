use std::collections::HashMap;
use std::rc::*;

use crate::actions::*;
use crate::buffer::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum KeyExecutorState {
    Normal,
    Insert,
    Visual,
    MotionPending,
    CharPending,
    Replace,
}

pub struct Mapping {
    pub keys: Vec<&'static str>,
    pub func: Rc<dyn Fn(&mut KeyExecutor, &mut Buffer)>
}

pub struct KeyExecutor {
    pub state: KeyExecutorState,
    pub mappings: HashMap<KeyExecutorState, Vec<Mapping>>,
    pub keypresses: Vec<String>
}

impl KeyExecutor {
    pub fn new() -> KeyExecutor {
        let mut ke = KeyExecutor {
            state: KeyExecutorState::Normal,
            mappings: HashMap::new(),
            keypresses: vec![]
        };
        let states: [KeyExecutorState; 6] = [
            KeyExecutorState::Normal,
            KeyExecutorState::Insert,
            KeyExecutorState::Visual,
            KeyExecutorState::MotionPending,
            KeyExecutorState::CharPending,
            KeyExecutorState::Replace];
        for state in states {
            ke.mappings.insert(state, vec![]);
        }
        ke.initialize_mappings();
        
        ke
    }
    
    pub fn execute_key(&mut self, key: &str, buffer: &mut Buffer) {
        self.keypresses.push(key.to_string());
        match self.state {
            KeyExecutorState::Normal => self.execute_normal(buffer),
            KeyExecutorState::Insert => self.execute_insert(buffer),
            KeyExecutorState::Visual => self.execute_normal(buffer),
            KeyExecutorState::MotionPending=> self.execute_normal(buffer),
            KeyExecutorState::CharPending => self.execute_normal(buffer),
            KeyExecutorState::Replace => self.execute_normal(buffer),
        }
    }
    
    fn search_mappings(&self, search_state: KeyExecutorState) -> (Option<usize>, bool) {
        let mut found_mapping: Option<usize> = None;
        let mut exists_mapping = false;
        
        let potential_mappings: &Vec<Mapping> = &self.mappings[&search_state];
        for (idx, mapping) in potential_mappings.iter().enumerate() {
            if mapping.keys.len() >= self.keypresses.len() {
                if mapping.keys[0..self.keypresses.len()].iter().zip(self.keypresses.iter()).all(|(a,b)| a==b) {
                    exists_mapping = true;
                }
            }
            if mapping.keys.len() == self.keypresses.len() {
                if mapping.keys.iter().zip(self.keypresses.iter()).all(|(a,b)| a==b) {
                    found_mapping = Some(idx);
                    break;
                }
            }
        }
        
        return (found_mapping, exists_mapping);
    }
    
    fn execute_normal(&mut self, buffer: &mut Buffer) {
        let (found_mapping, exists_mapping) = self.search_mappings(KeyExecutorState::Normal);
        
        if let Some(idx) = found_mapping {
            self.keypresses.clear();
            let func = self.mappings[&KeyExecutorState::Normal][idx].func.clone();
            func(self, buffer);
        }
        if !exists_mapping {
            self.keypresses.clear();
        }
    }
    
    fn execute_insert(&mut self, buffer: &mut Buffer) {
        if self.keypresses[0] == "Escape" {
            self.state = KeyExecutorState::Normal;
        } else if self.keypresses[0].len() == 1 {
            execute_action(insert_char, buffer, Some(self.keypresses[0].as_ref()));
            execute_action(move_right, buffer, None);
        }
        self.keypresses.clear();
    }
    
    fn initialize_mappings(&mut self) {
        let normal_mappings = self.mappings.get_mut(&KeyExecutorState::Normal).unwrap();
        normal_mappings.push(Mapping { keys: vec!["h"], func: Rc::new(|state, buffer| {
            execute_action(move_left, buffer, None);
        })});
        normal_mappings.push(Mapping { keys: vec!["j"], func: Rc::new(|state, buffer| {
            execute_action(move_down, buffer, None);
        })});
        normal_mappings.push(Mapping { keys: vec!["k"], func: Rc::new(|state, buffer| {
            execute_action(move_up, buffer, None);
        })});
        normal_mappings.push(Mapping { keys: vec!["l"], func: Rc::new(|state, buffer| {
            execute_action(move_right, buffer, None);
        })});
        normal_mappings.push(Mapping { keys: vec!["g","g"], func: Rc::new(|state, buffer| {
            execute_action(move_beginning, buffer, None);
        })});
        normal_mappings.push(Mapping { keys: vec!["x"], func: Rc::new(|state, buffer| {
            execute_action(delete_char, buffer, None);
        })});
        normal_mappings.push(Mapping { keys: vec!["i"], func: Rc::new(|state, buffer| {
            state.state = KeyExecutorState::Insert;
        })});
    }
}
