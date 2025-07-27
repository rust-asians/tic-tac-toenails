use crate::grid::Grid;
use crate::mark::Mark;

pub trait WinChecker {
    fn get_winner(grid: &impl Grid) -> Option<&impl Mark>;
}