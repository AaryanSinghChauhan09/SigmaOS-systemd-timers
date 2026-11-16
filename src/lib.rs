// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-timers: systemd timers component model.
//!
//! Real implementation: timers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timer {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdTimersError {
    EmptyTimerName,
    DuplicateTimer(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdTimersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTimerName => write!(f, "timer name must not be empty"),
            Self::DuplicateTimer(id) => write!(f, "duplicate timer {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdTimersError {}

pub struct SystemdTimers {
    pub name: String,
    timers: BTreeMap<String, Timer>,
    events: BTreeMap<String, Event>,
}

impl SystemdTimers {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            timers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_timer(&mut self, item: Timer) -> Result<(), SystemdTimersError> {
        if item.name.trim().is_empty() {
            return Err(SystemdTimersError::EmptyTimerName);
        }
        let id = item.id.clone();
        if self.timers.contains_key(&id) {
            return Err(SystemdTimersError::DuplicateTimer(id.clone()));
        }
        self.timers.insert(id, item);
        Ok(())
    }

    pub fn timer_count(&self) -> usize {
        self.timers.len()
    }

    pub fn get_timer(&self, id: &str) -> Option<&Timer> {
        self.timers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdTimersError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdTimersError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_timer(id: &str, name: &str) -> Timer {
        Timer {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdTimers::new("SystemdTimers");
        assert_eq!(m.name, "SystemdTimers");
        assert_eq!(m.timer_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_timer() {
        let mut m = SystemdTimers::new("SystemdTimers");
        m.add_timer(sample_timer("a1", "Alpha")).unwrap();
        assert_eq!(m.timer_count(), 1);
        assert!(m.get_timer("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdTimers::new("SystemdTimers");
        assert!(m.add_timer(sample_timer("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdTimers::new("SystemdTimers");
        m.add_timer(sample_timer("x", "One")).unwrap();
        assert!(m.add_timer(sample_timer("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdTimers::new("SystemdTimers");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
