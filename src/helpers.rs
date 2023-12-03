pub fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

pub fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

// returns (g, x, y) for a*x + b*y = g
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut max = a;
    let mut min = b;

    let mut prev_x: i64 = 1;
    let mut x: i64 = 0;
    let mut prev_y: i64 = 0;
    let mut y: i64 = 1;

    loop {
        let q = max / min;
        (x, prev_x) = (prev_x - q * x, x);
        (y, prev_y) = (prev_y - q * y, y);
        (max, min) = (min, max % min);

        if min == 0 {
            return (max, prev_x, prev_y);
        }
    }
}

// Extended gcd for multiple numbers
pub fn egcd_mn(numbers: &Vec<i64>) -> Option<(i64, Vec<i64>)> {
    if numbers.len() < 2 {
        return None;
    }
    let mut results: Vec<i64> = vec![1];
    let mut current = *numbers.first().unwrap();
    for n in numbers.iter().skip(1) {
        let (g, x, y) = egcd(current, *n);
        results = results.into_iter().map(|r| r * x).collect();
        results.push(y);
        current = g;
    }

    Some((current, results))
}

// Chinese Remainder Theorem
// https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Existence_.28direct_construction.29
pub fn crt(numbers_with_remainders: Vec<(i64, i64)>) -> i64 {
    let full_product: i64 = numbers_with_remainders.iter().map(|n| n.0).product();
    let mut result = 0;

    for (n, offset) in numbers_with_remainders.iter() {
        let product_without_n = full_product / n;
        let (_, inv, _) = egcd(product_without_n, *n);
        result += offset * product_without_n * inv;
    }

    result.rem_euclid(full_product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(20, 15), 5);
        assert_eq!(gcd(13, 17), 1);
    }

    #[test]
    fn test_egcd() {
        assert_eq!(egcd(20, 15), (5, 1, -1));
        assert_eq!(egcd(13, 17), (1, 4, -3));
    }

    #[test]
    fn test_egcd_mn() {
        assert_eq!(egcd_mn(&vec![20, 15, 10]), Some((5, vec![1, -1, 0])));
        assert_eq!(egcd_mn(&vec![19, 31, 59]), Some((1, vec![-13, 8, 0])));
    }
}
