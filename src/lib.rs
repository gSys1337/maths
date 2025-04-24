pub mod real;
pub mod integer;
pub mod naturals;
#[cfg(test)]
mod tests;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub enum Sign {
    Negative,
    Positive,
}
