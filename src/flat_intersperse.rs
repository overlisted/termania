/// this should get added to std
use std::iter::Peekable;

enum Step {
    PutNext,
    PutSeparator(usize),
}

pub struct FlatIntersperse<I: Iterator>
where
    I::Item: Clone,
{
    separator: Vec<I::Item>,
    iter: Peekable<I>,
    step: Step,
}

impl<I: Iterator> FlatIntersperse<I>
where
    I::Item: Clone,
{
    pub fn new(iter: I, separator: Vec<I::Item>) -> Self {
        Self {
            iter: iter.peekable(),
            separator,
            step: Step::PutNext,
        }
    }
}

impl<I> Iterator for FlatIntersperse<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.peek().is_none() {
            None
        } else {
            match self.step {
                Step::PutNext => {
                    if !self.separator.is_empty() {
                        self.step = Step::PutSeparator(0);
                    }

                    self.iter.next()
                }
                Step::PutSeparator(i) => {
                    let next_i = i + 1;

                    self.step = if next_i == self.separator.len() {
                        Step::PutNext
                    } else {
                        Step::PutSeparator(next_i)
                    };

                    Some(self.separator[i].clone())
                }
            }
        }
    }
}
