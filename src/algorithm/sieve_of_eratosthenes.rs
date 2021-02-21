pub struct Eratosthenes {
    table: Vec<u64>,
}

impl Eratosthenes {
    pub fn new(l: usize) -> Eratosthenes {
        let mut nums = vec![true; l];
        nums[0] = false;
        nums[1] = false;
        for i in 2..l {
            if nums[i] {
                let mut j = 2;
                while i * j < l {
                    nums[i * j] = false;
                    j += 1;
                }
            }
        }
        Eratosthenes {
            table: nums
                .iter()
                .enumerate()
                .filter(|(_, &f)| f)
                .map(|(i, _)| i as u64)
                .collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Eratosthenes;
    #[test]
    fn test() {
        let a = Eratosthenes::new(10);
        assert_eq!(a.table, vec![2, 3, 5, 7]);
    }
}
