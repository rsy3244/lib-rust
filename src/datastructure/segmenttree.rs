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

#[derive(Clone)]
struct SegmentTree<T: Monoid>{
    size: usize,
    data: Vec<T>,
}

impl<T> SegmentTree<T>
where T: Monoid,
      T: Copy
{
    fn new(n: usize) -> Self {
        SegmentTree{
            size : n,
            data : vec![T::unit(); 2*n+1],
        }
    }

    fn update(&mut self, idx: usize, e: T) {
        let mut idx = idx << 1;
        while idx > 0 {
            self.data[idx] = self.data[idx].ope(e);
            idx >>= 1;
        }
    }
    
    fn querry(&self, l: usize, r: usize) -> Result<T, &'static str> {
        if r == 0 { return Err("r must be nonzero value.") }
        let mut res = T::unit();
        let mut l = l << 1;
        let mut r = r << 1;
        while l < r {
            if l & 1 == 1 { res = res.ope(self.data[l]); l += 1;}
            if r & 1 == 1 { res = res.ope(self.data[r-1]); r-= 1;}
            l <<= 1; r <<= 1;
        }
        Ok(res)
    }
}

impl Monoid for u64 {
    fn unit() -> u64 { 0 }
    fn ope(self, e: u64) -> u64 { self + e }
    //fn trailing_zeros(self) -> u32 { self.trailing_zeros() }
}

fn main(){
	let cin = stdin();
	let cin = cin.lock();
	let mut sc = Scanner::new(cin);
	let n : usize = sc.read();
    let mut st = SegmentTree::<u64>::new(n);

	
}

