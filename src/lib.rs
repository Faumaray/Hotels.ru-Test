use std::collections::{HashMap, HashSet};
//5min
mod city;
//15min

/// Находит элементы повторяющиеся в обоих массивах 2 и больше раза.
pub fn find_more_than_two_occuraces(
    first: impl AsRef<[i32]>,
    second: impl AsRef<[i32]>,
) -> Vec<i32> {
    let first_occuraces: HashSet<i32> = first
        .as_ref()
        .into_iter()
        .fold(HashMap::<i32, usize>::new(), |mut m, x| {
            *m.entry(*x).or_default() += 1;
            m
        }) // Находим количество появлений чисел в массиве 1
        .into_iter()
        .filter(|entry| entry.1 >= 2) // Убираем все числа что встречаются меньше 2 раз
        .map(|entry| entry.0) // Забираем только их значения
        .collect();

    let second_occuraces: HashSet<i32> = second
        .as_ref()
        .into_iter()
        .fold(HashMap::<i32, usize>::new(), |mut m, x| {
            *m.entry(*x).or_default() += 1;
            m
        }) // Находим количество появлений чисел в массиве 2
        .into_iter()
        .filter(|entry| entry.1 >= 2) // Убираем все числа что встречаются меньше 2 раз
        .map(|entry| entry.0) // Забираем только их значения
        .collect();

    let mut intersection = first_occuraces
        .intersection(&second_occuraces) // Находим пересечения
        .cloned()
        .collect::<Vec<i32>>();
    intersection.sort();
    intersection
}
//3min
/// Округляет число до ближайшего числа делимого на заданное
/// `number`: Чилос для округления
/// `floor`: Делитель
pub fn floor_to_n(number: f32, floor: usize) -> i32 {
    ((number / floor as f32).round() as i32 * floor as i32) as i32
}

/// Округление числа до ближайшего 5
pub fn floor_to_five(number: f32) -> i32 {
    floor_to_n(number, 5)
}
//8min
/// Склоняет слово компьютер относильного заданного числа
pub fn computer_in_case(count: i32) -> impl AsRef<str> {
    if (11..=14).contains(&(count.abs() % 100)) {
        return format!("{} компьютеров", count);
    }

    match count.abs() % 10 {
        1 => format!("{} компьютер", count),
        2..=4 => format!("{} компьютера", count),
        _ => format!("{} компьютеров", count),
    }
}
//5min
/// Проверяет является ли число простым
pub fn is_prime(number: i32) -> bool {
    if (0..=1).contains(&number) {
        return false;
    }
    let limit = ((number as f64).sqrt() as i32).abs();

    for i in 2..=limit {
        if number % i == 0 {
            return false;
        }
    }

    true
}
//20min
/// Создает строку содержащую таблицу умножения до заданного числа
pub fn get_multiplication_table_string(limit: usize) -> impl AsRef<str> {
    let mut table: Vec<Vec<String>> = vec![vec![String::new(); limit + 1]; limit + 1];

    for column in (0..=limit).rev() {
        let space = if column == 0 {
            (limit.checked_ilog10().unwrap_or(0) + 1) as usize
        } else {
            ((column * limit).checked_ilog10().unwrap_or(0) + 1) as usize
        };
        for row in (0..=limit).rev() {
            table[row][column] = if row == 0 && column != 0 {
                format!("{:>space$}", column)
            } else if column == 0 && row != 0 {
                format!("{:>space$}", row)
            } else if row != 0 && column != 0 {
                format!("{:>space$}", column * row)
            } else {
                format!("{:>space$}", ' ')
            };
        }
    }
    let table: Vec<String> = table.iter().map(|vc| vc.join(" ")).collect();
    table.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     *  1 2
     *1 1 2
     *2 2 4
     */
    #[test]
    fn test_multiplication_table_single() {
        assert_eq!(
            "  1 2\n1 1 2\n2 2 4",
            get_multiplication_table_string(2).as_ref()
        );
    }
    /*
     *  1  2  3  4  5
     *1 1  2  3  4  5
     *2 2  4  6  8 10
     *3 3  6  9 12 15
     *4 4  8 12 16 20
     *5 5 10 15 20 25
     */
    #[test]
    fn test_multiplication_table_double() {
        assert_eq!("  1  2  3  4  5\n1 1  2  3  4  5\n2 2  4  6  8 10\n3 3  6  9 12 15\n4 4  8 12 16 20\n5 5 10 15 20 25", get_multiplication_table_string(5).as_ref());
    }
    /*
     *    1  2  3  4  5  6  7  8  9  10  11
     * 1  1  2  3  4  5  6  7  8  9  10  11
     * 2  2  4  6  8 10 12 14 16 18  20  22
     * 3  3  6  9 12 15 18 21 24 27  30  33
     * 4  4  8 12 16 20 24 28 32 36  40  44
     * 5  5 10 15 20 25 30 35 40 45  50  55
     * 6  6 12 18 24 30 36 42 48 54  60  66
     * 7  7 14 21 28 35 42 49 56 63  70  77
     * 8  8 16 24 32 40 48 56 64 72  80  88
     * 9  9 18 27 36 45 54 63 72 81  90  99
     *10 10 20 30 40 50 60 70 80 90 100 110
     *11 11 22 33 44 55 66 77 88 99 110 121
     */
    #[test]
    fn test_multiplication_table_triple() {
        assert_eq!("    1  2  3  4  5  6  7  8  9  10  11\n 1  1  2  3  4  5  6  7  8  9  10  11\n 2  2  4  6  8 10 12 14 16 18  20  22\n 3  3  6  9 12 15 18 21 24 27  30  33\n 4  4  8 12 16 20 24 28 32 36  40  44\n 5  5 10 15 20 25 30 35 40 45  50  55\n 6  6 12 18 24 30 36 42 48 54  60  66\n 7  7 14 21 28 35 42 49 56 63  70  77\n 8  8 16 24 32 40 48 56 64 72  80  88\n 9  9 18 27 36 45 54 63 72 81  90  99\n10 10 20 30 40 50 60 70 80 90 100 110\n11 11 22 33 44 55 66 77 88 99 110 121", get_multiplication_table_string(11).as_ref());
    }

    #[test]
    fn test_occuraces() {
        assert_eq!(
            vec![1, 17],
            find_more_than_two_occuraces(
                [7, 17, 1, 9, 1, 17, 56, 56, 23],
                [56, 17, 17, 1, 23, 34, 23, 1, 8, 1]
            )
        );
    }

    #[test]
    fn test_prime() {
        assert!(is_prime(2));
        assert!(is_prime(4957));
        assert!(is_prime(5));
        assert_eq!(false, is_prime(0));
        assert_eq!(false, is_prime(21));
        assert_eq!(false, is_prime(1))
    }

    #[test]
    fn test_computer_cases() {
        assert_eq!("25 компьютеров", computer_in_case(25).as_ref());
        assert_eq!("41 компьютер", computer_in_case(41).as_ref());
        assert_eq!("11 компьютеров", computer_in_case(11).as_ref());
        assert_eq!("111 компьютеров", computer_in_case(111).as_ref());
        assert_eq!("112 компьютеров", computer_in_case(112).as_ref());
        assert_eq!("1011 компьютеров", computer_in_case(1011).as_ref());
        assert_eq!("2043 компьютера", computer_in_case(2043).as_ref());
    }

    #[test]
    fn floor_test() {
        assert_eq!(floor_to_five(27.0), 25);
        assert_eq!(floor_to_five(27.8), 30);
        assert_eq!(floor_to_five(41.7), 40);
        assert_eq!(floor_to_five(-41.7), -40);
        assert_eq!(floor_to_n(100.0, 30), 90);
        assert_eq!(floor_to_n(100.0, 40), 120);
        assert_eq!(floor_to_n(-100.0, 40), -120);
    }
}
