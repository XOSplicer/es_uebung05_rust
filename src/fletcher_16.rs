use cast::cast_slice_to_bytes;

pub struct Fletcher16Hasher {
    sum1: u16,
    sum2: u16,
}

impl Fletcher16Hasher {
    pub fn new() -> Fletcher16Hasher {
        Fletcher16Hasher {
            sum1: 0,
            sum2: 0,
        }
    }
    pub fn write_u8(&mut self, data: u8) {
        self.sum1 = (self.sum1 + data as u16) % 255;
        self.sum2 = (self.sum2 + self.sum1) % 255;
    }
    pub fn write_u16_platform(&mut self, data: u16) {
        // plattform dependent :(
        let buf: &[u16] = &[data];
        let buf: &[u8] = unsafe {
            cast_slice_to_bytes(buf)
        };
        for x in buf {
            self.write_u8(*x);
        }
    }
    pub fn finish(&self) -> u16 {
        (self.sum2 << 8) | self.sum1
    }
}

pub fn fletcher_16(data: &[u8]) -> u16 {
    let mut hasher = Fletcher16Hasher::new();
    for x in data {
        hasher.write_u8(*x);
    }
    hasher.finish()
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