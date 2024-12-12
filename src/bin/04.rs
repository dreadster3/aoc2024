advent_of_code::solution!(4);

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let length = matrix[0].len();

    (0..length)
        .map(|i| {
            matrix
                .iter()
                .map(|r| r.get(i).unwrap().clone())
                .rev()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

fn get_diagonal(matrix: Vec<Vec<char>>) -> String {
    assert!(matrix.len() == matrix[0].len());

    let mut diagonal = Vec::new();
    let length = matrix.len();

    for i in 0..length {
        for j in 0..length {
            if i == j {
                diagonal.push(matrix[i][j]);
            }
        }
    }

    return diagonal.iter().collect::<String>();
}

fn count_xmas(matrix: Vec<Vec<char>>) -> u32 {
    assert!(matrix.len() == 4);
    assert!(matrix.len() == matrix[0].len());
    let transpose = transpose(matrix.clone());

    let first_row = matrix[0].iter().collect::<String>();
    let first_column = transpose[0].iter().collect::<String>();
    let diagonal_1 = get_diagonal(matrix.clone());
    let diagonal_2 = get_diagonal(transpose.clone());

    [first_row, first_column, diagonal_1, diagonal_2]
        .iter()
        .filter(|&w| w.eq("XMAS") || w.eq("SAMX"))
        .count() as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut result = 0;
    for i in 0..matrix.len() - 3 {
        for j in 0..matrix.len() - 3 {
            let square = matrix[i..i + 4]
                .iter()
                .enumerate()
                .map(|(offset, _)| matrix[i + offset][j..j + 4].to_vec())
                .collect::<Vec<Vec<char>>>();

            let count = count_xmas(square);

            result += count;
        }
    }

    let last_rows: usize = matrix
        .iter()
        .skip(matrix.len() - 3)
        .map(|r| r.iter().collect::<String>())
        .map(|s| s.matches("XMAS").count() + s.matches("SAMX").count())
        .sum();

    let last_columns: usize = transpose(
        matrix
            .iter()
            .map(|r| {
                r.iter()
                    .skip(matrix.len() - 3)
                    .map(|x| x.to_owned())
                    .collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>(),
    )
    .iter()
    .map(|c| c.iter().collect::<String>())
    .map(|s| s.matches("XMAS").count() + s.matches("SAMX").count())
    .sum();

    result += last_columns as u32;
    result += last_rows as u32;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
