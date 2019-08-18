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

mod vebtree{
    struct VEBtree {
        size : usize,
        root : VEBnode,
    }

    struct VEBnode {
        type Num = u32;

        min: Num,
        max: Num,
        summary : VEBnode,
        cluster : [VEBnode],
    }

    impl VEBtree {
        #![inline]
        fn new(u: usize) -> VEBtree {
            VEBtree{
                size = 0,
                root = VEBtree.new(u),
            }
        }
        fn min(&self) { self.root.min() }
        fn max(&self) { self.root.max() }
        fn size(&self) { size() }
        fn find(&self, e) { self.root.find(e) }
        fn insert(&mut self, e) { if !self.find(e) { self.size += 1; self.root.insert(e) }
            else { Err("already exist.") }
        }
        fn delete(&mut self, e) { if self.find(e) { self.size -= 1; self.root.delete(e) }
            else { Err("does not exist.") }
        }
    }

    impl VEBnode {
        fn new(u: usize) -> VEBnode {
            VEBnode {
                min = 1,
                max = 0,
                summary = VEBnode.new(u.sqrt())
}

fn main(){
	let cin = stdin();
	let cin = cin.lock();
	let mut sc = Scanner::new(cin);
	let n : usize = sc.read();
	
}

