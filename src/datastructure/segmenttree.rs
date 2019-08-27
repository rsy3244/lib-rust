use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;
use std::cmp;

struct Scanner<'a> {
    cin : StdinLock<'a>,
}

impl<'a> Scanner<'a> {
    fn new(cin : StdinLock<'a>) -> Scanner<'a> {
        Scanner { cin: cin }
    }

    fn read1<T: FromStr>(&mut self) -> Option<T> {
        let token = self.cin.by_ref().bytes().map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        token.parse::<T>().ok()
    }

    fn read<T: FromStr>(&mut self) -> T {
        self.read1().unwrap()
    }
}

trait Monoid {
    fn unit() -> Self;
    fn ope(self, e: Self) -> Self;
    //fn shl(self, e: u32) -> Self;
    //fn trailing_zeros(self) -> u32;
    //fn trailing_nonzeros(self) -> u32;
}

#[derive(Clone, Debug)]
struct SegmentTree<T: Monoid>{
    size: usize,
    data: Vec<T>,
}

impl<T> SegmentTree<T>
where T: Monoid,
      T: Copy,
      T: std::fmt::Debug,
{
    fn new(n: usize) -> Self {
        SegmentTree{
            size : n,
            data : vec![T::unit(); 2*n+1],
        }
    }

    fn update(&mut self, idx: usize, e: T) {
        let mut idx = idx + self.size;
        while idx > 0 {
            self.data[idx] = self.data[idx].ope(e);
            idx >>= 1;
            //println!("{:?}", self.data);
        }
    }
    
    fn querry(&self, l: usize, r: usize) -> Result<T, &'static str> {
        if r == 0 { return Err("r must be nonzero value.") }
        let mut res = T::unit();
        let mut l = l + self.size;
        let mut r = r + self.size;
        //println!("{:?}", self.data);
        while l < r {
            //println!("[{},{})", l, r);
            if l & 1 == 1 { res = res.ope(self.data[l]); l += 1;}
            if r & 1 == 1 { res = res.ope(self.data[r-1]); r-= 1;}
            l >>= 1; r >>= 1;
        }
        Ok(res)
    }
}

impl Monoid for u32 {
    fn unit() -> u32 { 0 }
    fn ope(self, e: u32) -> u32 { self + e }
    //fn trailing_zeros(self) -> u32 { self.trailing_zeros() }
}

/*
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let n : usize = sc.read();
    let q : usize = sc.read();
    let mut st = SegmentTree::<u32>::new(n);
    for _ in 0..q {
        match sc.read() {
            0 => {
                let (x, y) : (usize, u32) = (sc.read(), sc.read());
                st.update(x-1, y);
            },
            _ => {
                let (x, y) : (usize, usize) = (sc.read(), sc.read());
                println!("{}", st.querry(x-1, y).ok().unwrap());
            },
        }
    }
}
*/

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn processing_querry(){
        let mut st = SegmentTree::new(11);
        assert_eq!(st.querry(0,11),Ok(0));
        st.update(5, 10);
        st.update(1, 8);
        assert_eq!(st.querry(0,5), Ok(8));
        assert_eq!(st.querry(0,11),Ok(18));
    }
}

