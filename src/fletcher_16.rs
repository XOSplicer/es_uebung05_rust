pub fn fletcher_16(data: &[u8]) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;
    for x in data {
        sum1 = (sum1 + *x as u16) % 255;
        sum2 = (sum2 + sum1) % 255;
    }
    (sum2 << 8) | sum1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_1() {
        assert_eq!(51440 , fletcher_16("abcde".as_bytes()));
    }
    #[test]
    fn test_vector_2() {
        assert_eq!(8279, fletcher_16("abcdef".as_bytes()));
    }
    #[test]
    fn test_vector_3() {
        assert_eq!(1575 , fletcher_16("abcdefgh".as_bytes()));
    }
}