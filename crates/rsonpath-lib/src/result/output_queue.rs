//! Space-efficient output queue of nodes.
//!
//! When outputting results we need to maintain the specification ordained ordering,
//! which is not the same as the order in which we finalize matches.
//!
//! This queue allows storing match data in the correct order based on an id and
//! output all of the queued nodes at once.
use super::Sink;

/// Queue of arbitrary output data.
pub(super) struct OutputQueue<D> {
    offset: usize,
    nodes: Vec<Option<D>>,
}

impl<D> OutputQueue<D> {
    /// Create an empty [`OutputQueue`].
    pub(super) fn new() -> Self {
        Self {
            offset: 0,
            nodes: vec![],
        }
    }

    /// Insert output data associated with the given `id`.
    ///
    /// The ids have to be unique, and have to cover the entire
    /// space from 0 to the number of calls to `insert` - 1.
    pub(super) fn insert(&mut self, id: usize, node: D) {
        let actual_idx = id - self.offset;

        while self.nodes.len() <= actual_idx {
            self.nodes.push(None);
        }

        self.nodes[actual_idx] = Some(node);
    }

    /// Output all queued data in the id order.
    ///
    /// It is assumed that nodes come in batches with no holes in the ids.
    /// In other words, if `insert` is called with id N, `output_to` can only be called
    /// after nodes with ids 0 through N - 1 have also been `insert`ed.
    ///
    /// These sequences of calls are valid:
    /// - `insert(0)`, `insert(1)`, `insert(2)`, `output_to`, `insert(4)`, `insert(3)`, `output_to`
    /// - `insert(2)`, `insert(3)`, `insert(1)`, `insert(0)`, `output_to`
    ///
    /// These sequences are *invalid*:
    /// - `insert(1)`, `output_to`
    /// - `insert(1)`, `insert(0)`, `output_to`, `insert(3)` `output_to`
    pub(super) fn output_to<S>(&mut self, sink: &mut S) -> Result<(), S::Error>
    where
        S: Sink<D>,
    {
        self.offset += self.nodes.len();

        for node in self.nodes.drain(..) {
            sink.add_match(node.expect("output_to called only after all matches are complete"))?;
        }

        Ok(())
    }
}
