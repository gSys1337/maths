pub mod real;
pub mod integer;
#[cfg(test)]
mod tests;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub enum Sign {
    Negative,
    Positive,
}
