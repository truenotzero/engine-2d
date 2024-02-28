use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

#[derive(Default)]
pub struct EventManager<E: Hash + Eq + Copy, D> {
    subscribers: HashMap<E, Vec<Sender<Rc<D>>>>,
    notifiers_rx: HashMap<E, Receiver<D>>,
    notifiers_tx: HashMap<E, Sender<D>>,
}

impl<E: Hash + Eq + Copy, D> EventManager<E, D> {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
            notifiers_rx: HashMap::new(),
            notifiers_tx: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event: E) -> Receiver<Rc<D>> {
        let (tx, rx) = mpsc::channel();

        let mut subs = self.subscribers.remove(&event).unwrap_or_default();
        subs.push(tx);
        self.subscribers.insert(event, subs);

        rx
    }

    pub fn make_notifier(&mut self, event: E) -> Sender<D> {
        if self.notifiers_tx.get(&event).is_none() {
            let (tx, rx) = mpsc::channel();
            self.notifiers_tx.insert(event, tx);
            self.notifiers_rx.insert(event, rx);
        }

        self.notifiers_tx.get(&event).unwrap().clone()
    }

    pub fn tick(&mut self) {
        let mut cleanup = HashMap::new();

        for (event, notifier) in self.notifiers_rx.iter() {
            if let Ok(data) = notifier.try_recv() {
                let data = Rc::new(data);
                if let Some(subs) = self.subscribers.get(event) {
                    for (idx, sub) in subs.iter().enumerate() {
                        if sub.send(data.clone()).is_err() {
                            if cleanup.get(&event).is_none() {
                                cleanup.insert(event, Vec::new());
                            }
                            cleanup.get_mut(&event).unwrap().push(idx);
                        }
                    }
                }
            }
        }

        // do cleanup of dropped subscribers
        for (event, indices) in cleanup {
            for i in indices {
                self.subscribers.get_mut(event).unwrap().swap_remove(i);
            }
        }
    }
}
