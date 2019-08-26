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
    fn ope(&self, e: Self) -> Self;
}

struct SegmentTree<T: Monoid>{
    size: usize,
    data: Vec<T>,
}

impl<T> SegmentTree<T>
where T: Monoid,
      T: Clone
{
    fn new(n: usize) -> Self {
        SegmentTree{
            size : n,
            data : vec![T::unit(); 2*n+1],
        }
    }
    fn update(&mut self, idx: usize, e: T) {
        let mut idx = e*2;
        while idx > 0 {
            data[idx] = data[idx].ope(e);
            idx /= 2;
        }
    }
    
    fn querry(l: mut usize, r: mut usize) -> T {
        let mut res = T.unit();
        l *= 2; r *= 2;
        while l < r {
            res = res.ope(data[l]);
            
            
        
        
}

impl Monoid for u64 {
    fn unit() -> u64 { 0 }
    fn ope(&self, e: u64) -> u64 { self + e }
}


fn main(){
	let cin = stdin();
	let cin = cin.lock();
	let mut sc = Scanner::new(cin);
	let n : usize = sc.read();
    let mut st = SegmentTree::<u64>::new(n);

	
}

