use vectrix::Matrix;

pub type BoardMatrix = Matrix<(u32, bool), 5, 5>;

#[derive(Debug, Clone, Copy)]
pub struct Board {
    numbers: BoardMatrix,
}
impl Board {
    pub fn new(numbers: BoardMatrix) -> Board {
        Board { numbers }
    }

    pub fn mark_number(&mut self, called_number: u32) {
        self.numbers = self
            .numbers
            .iter()
            .map(|x| {
                if x.0 == called_number {
                    return (x.0, true);
                }
                *x
            })
            .collect()
    }

    pub fn is_winner(&self) -> bool {
        let row_full = self.numbers.iter_rows().any(|row| row.iter().all(|x| x.1));

        let col_full = self
            .numbers
            .iter_columns()
            .any(|col| col.iter().all(|x| x.1));

        row_full || col_full
    }

    pub fn score(&self, called_number: u32) -> u32 {
        self.numbers
            .into_iter()
            .filter(|x| !x.1)
            .map(|x| x.0)
            .sum::<u32>()
            * called_number
    }
}
