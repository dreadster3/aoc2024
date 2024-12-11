advent_of_code::solution!(4);

fn count_xmas_rows(matrix: Vec<Vec<char>>) -> u32 {
    assert!(matrix.len() == 4);
    assert!(matrix.len() == matrix[0].len());
    matrix
        .iter()
        .map(|r| r.iter().collect::<String>())
        .filter(|w| w.eq("XMAS") || w.eq("SAMX"))
        .count() as u32
}

fn count_xmas_diagonal(matrix: Vec<Vec<char>>) -> u32 {
    assert!(matrix.len() == 4);
    assert!(matrix.len() == matrix[0].len());

    let length = matrix.len();
    let mut diagonal_1 = Vec::new();
    let mut diagonal_2 = Vec::new();

    for i in 0..length {
        for j in 0..length {
            if i == j {
                diagonal_1.push(matrix[i][j]);
            }
            if i + j == length - 1 {
                diagonal_2.push(matrix[i][j]);
            }
        }
    }

    let word1 = diagonal_1.iter().collect::<String>();
    let word2 = diagonal_2.iter().collect::<String>();

    [word1, word2]
        .iter()
        .filter(|&w| w.eq("XMAS") || w.eq("SAMX"))
        .count() as u32
}

fn count_xmas(matrix: Vec<Vec<char>>) -> (u32, u32, u32) {
    assert!(matrix.len() == 4);
    assert!(matrix.len() == matrix[0].len());
    let length = matrix.len();

    let flip = (0..length)
        .map(|i| {
            matrix
                .iter()
                .map(|r| r.get(i).unwrap().clone())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let row = matrix[0].iter().collect::<String>();
    let column = flip[0].iter().collect::<String>();
    let rows = match row.as_str() {
        "XMAS" => 1,
        "SAMX" => 1,
        _ => 0,
    };
    let diagonal = count_xmas_diagonal(matrix.clone());
    let columns = match column.as_str() {
        "XMAS" => 1,
        "SAMX" => 1,
        _ => 0,
    };

    (rows, columns, diagonal)
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

            let (rows, cols, diagonal) = count_xmas(square);

            result += rows + cols + diagonal;
        }
    }

    let last_row = matrix.iter().last().unwrap().iter().collect::<String>();

    let last_column = matrix.iter().map(|c| c.last().unwrap()).collect::<String>();

    result += (last_row.matches("XMAS").count() + last_row.matches("SAMX").count()) as u32;
    result += (last_column.matches("XMAS").count() + last_column.matches("SAMX").count()) as u32;

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
