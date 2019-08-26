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

mod union_find{
    struct Uf{
        par: Vec<usize>,
        rank: Vec<usize>,
    }

    impl Uf{
        pub fn new(u: usize) -> Uf{
            let mut par = vec![0 ; u];
            for (idx, e) in par.iter_mut().enumerate() {
                *e = idx;
            }
            Uf{
                par,
                rank : vec![0 ; u],
            }
        }

        pub fn find(&self, u: usize) -> Uf{
            if self.par[u] == u { u }
            else { self.par[u] = self.find(u); self.par[u] }
        }

        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            let x = self.find(x);
            let y = self.find(y);
            if x == y { false }

            if *self.rank[x] < *self.rank[y] { self.par[x] = y; }
            else {
                self.par[y] = self.par[x];
                if self.rank[x] == self.rank[y] { self.rank[y] += 1; }
            }
            true
        }

        #[inline]
        pub fn same(&self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
    }
}


fn main(){
	let cin = stdin();
	let cin = cin.lock();
	let mut sc = Scanner::new(cin);

    let mut u = union_find::new(16);
    println!("{}", u.same(0,1));


	
}

