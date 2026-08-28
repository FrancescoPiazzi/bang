// Current Chooser interface clones elements, this has the potential to suck
// because not only cloning is boring because it doesn't make you mentally
// insane by dragging a million lifetimes everywhere, it also means
// you can't perform an operation on a randomly chosen element, modify it
// have the change reflected on the original one
//
// This (probably) solves all of these problems, but I don't really need it now

use std::cell::Cell;
use rand::seq::IndexedRandom;

// 1. Clear out trait lifetimes. 
// 'b is the lifetime of the Vec elements; 'a is the temporary lifetime of the method call.
pub(crate) trait Chooser<T> {
    fn choose<'a, 'b>(&'a self, container: &'b Vec<T>) -> Option<&'b T>;
}

// --- REAL IMPLEMENTATION ---
struct RandomChooser;

impl<T> Chooser<T> for RandomChooser {    
    fn choose<'a, 'b>(&'a self, container: &'b Vec<T>) -> Option<&'b T> {
        container.choose(&mut rand::rng())
    }
}

// --- MOCK IMPLEMENTATION ---
#[derive(Clone, Copy, Debug, PartialEq)]
enum DieFace { One, Two, Three }

struct LoopingChooser {
    // 2. Use Cell for simple primitives like usize so we can mutate via &self
    count: Cell<usize>,
    looped_values: Vec<DieFace>,
}

impl LoopingChooser {
    fn new(looped_values: Vec<DieFace>) -> LoopingChooser {
        LoopingChooser {
            count: Cell::new(0),
            looped_values,
        }
    }
}

impl Chooser<DieFace> for LoopingChooser {    
    fn choose<'a, 'b>(&'a self, _: &'b Vec<DieFace>) -> Option<&'b DieFace> {
        let current_count = self.count.get();
        let nxt = &self.looped_values[current_count];

        // 3. Update the count safely without needing &mut self
        let next_count = if current_count + 1 < self.looped_values.len() {
            current_count + 1
        } else {
            0
        };
        self.count.set(next_count);
        
        Some(nxt)
    }
}