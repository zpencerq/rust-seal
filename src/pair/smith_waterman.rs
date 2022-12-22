use std::cmp;

use crate::pair::step_mask::StepMask;
use crate::pair::strategy::Strategy;

#[derive(Clone, Debug)]
pub struct SmithWaterman {
    align: isize,
    insert: isize,
    delete: isize,
}

impl SmithWaterman {
    pub fn new(align: isize, insert: isize, delete: isize) -> SmithWaterman {
        SmithWaterman {
            align,
            insert,
            delete,
        }
    }
}

impl Strategy for SmithWaterman {
    fn mismatch_score(&self) -> isize {
        self.align
    }

    fn insert_score(&self) -> isize {
        self.insert
    }

    fn delete_score(&self) -> isize {
        self.delete
    }

    fn total_score(&self, score: isize) -> isize {
        if score >= 0 {
            score
        } else {
            0
        }
    }

    fn step_mask(&self, align: isize, insert: isize, delete: isize) -> StepMask {
        if cmp::max(cmp::max(align, insert), delete) > 0 {
            StepMask::from_scores(align, insert, delete)
        } else {
            StepMask::STOP
        }
    }
}
