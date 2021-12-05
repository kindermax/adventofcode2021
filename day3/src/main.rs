const DATA: &'static str = include_str!("../input.txt");


fn read_binary_numbers(raw_data: &str) -> Vec<String> {
    raw_data.split_whitespace().map(|s| s.into()).collect()
}

fn calculate_gamma_and_epsilon_rate(binary_numbers: &Vec<String>) -> (usize, usize) {
    let bits_count: usize = binary_numbers[0].len();
    let mut gamma_binary = String::from("");
    let mut epsilon_binary = String::from("");

    let mut zero_count = 0;

    for idx in 0..bits_count {
        for number in binary_numbers {
            let nth_char = number.chars().nth(idx).unwrap();
            if nth_char == '0' {
                zero_count += 1;
            }
        }
        if binary_numbers.len() / 2 > zero_count  {
            gamma_binary.push_str("1");
            epsilon_binary.push_str("0");
        } else {
            gamma_binary.push_str("0");
            epsilon_binary.push_str("1");
        }

        zero_count = 0;
    }

    let gamma_rate = usize::from_str_radix(gamma_binary.as_ref(), 2).unwrap_or(0);
    let epsilon_rate = usize::from_str_radix(epsilon_binary.as_ref(), 2).unwrap_or(0);
    (gamma_rate, epsilon_rate)
}


enum Rating {
    Oxygen,
    Co2
}

impl Rating {
    fn apply(&self, binary_numbers: &Vec<String>) -> usize {
        use self::Rating::*;

        let mut numbers_with_zero: Vec<String> = Vec::new();
        let mut numbers_with_one: Vec<String> = Vec::new();

        let mut numbers: Vec<String> = binary_numbers.iter().map(|n| n.into()).collect();
        let mut idx = 0;
        while numbers.len() != 1 {
            for number in &numbers {
                let bit = number.chars().nth(idx).expect("invalid char at index 0");
                if bit == '0' {
                    numbers_with_zero.push(number.clone());
                } else {
                    numbers_with_one.push(number.clone());
                }
            }
            
            numbers = match self {
                Oxygen => self.choose_most_common(&numbers_with_one, &numbers_with_zero),
                Co2 => self.choose_least_common(&numbers_with_one, &numbers_with_zero),
            };

            numbers_with_one.clear();
            numbers_with_zero.clear();
            idx += 1;
        }
        
        usize::from_str_radix(&numbers[0], 2).unwrap()
    }

    fn choose_most_common(&self, a: &Vec<String>, b: &Vec<String>) -> Vec<String> {
        if a.len() >= b.len() {
            a.iter().map(|n| n.into()).collect()
        } else {
            b.iter().map(|n| n.into()).collect()
        }
    }

    fn choose_least_common(&self, a: &Vec<String>, b: &Vec<String>) -> Vec<String> {
        if a.len() < b.len() {
            a.iter().map(|n| n.into()).collect()
        } else {
            b.iter().map(|n| n.into()).collect()
        }
    }

}


fn main() {
    let binary_numbers = read_binary_numbers(DATA);
    let (gamma_rate, epsilon_rate) = calculate_gamma_and_epsilon_rate(&binary_numbers);

    println!("Gamma rate {}, epsilon_rate {}, power consumption {}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);

    let oxygen_rating = Rating::Oxygen.apply(&binary_numbers);
    let co2_rating = Rating::Co2.apply(&binary_numbers);


    println!("Oxygen rating {}, co2 rating {}, support rating {}", oxygen_rating, co2_rating, oxygen_rating * co2_rating);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_binary_numbers() {
        let binary_numbers = read_binary_numbers(r"
        001
        010
    ");

        assert_eq!(binary_numbers, vec!["001", "010"]);
    }

    
    #[test]
    fn test_calculate_gamma_and_epsilon_rate() {
        let binary_numbers = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010"
        ].iter().map(|s| (*s).into()).collect();

        let (gamma_rate, epsilon_rate) = calculate_gamma_and_epsilon_rate(&binary_numbers);

        assert_eq!(gamma_rate, 22);
        assert_eq!(epsilon_rate, 9);
    }
    
    #[test]
    fn test_get_oxygen_gen_rating() {
        let binary_numbers = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010"
        ].iter().map(|s| (*s).into()).collect();

        let oxygen_rating = Rating::Oxygen.apply(&binary_numbers);

        assert_eq!(oxygen_rating, 23);
    }

    #[test]
    fn test_get_co2_scrubber_rating() {
        let binary_numbers = vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010"
        ].iter().map(|s| (*s).into()).collect();

        let co2_rating = Rating::Co2.apply(&binary_numbers);

        assert_eq!(co2_rating, 10);
    }
}