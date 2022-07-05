mod cc_rs_libc_test;
mod include_test;
#[macro_use]
mod macro_test;
mod compare_option;
mod data_representation;
mod lifetime_test;
mod map_bug;
mod mutable_immutable;
mod option_test;

mod language_learn_cc_rs_libc_test_lib {
    use crate::cc_rs_libc_test::{another_function, show_some_content};

    pub fn call(input: i32) -> i32 {
        unsafe { another_function(input) as i32 }
    }

    pub fn call_show() {
        unsafe {
            show_some_content();
        }
    }
}

mod test_in_mod {
    #[test]
    fn some() {
        println!("some");
        assert!(false);
    }

    #[test]
    fn test() {
        macro_expr!(1, 2, 3);
        assert!(false);
    }
}

use language_learn_cc_rs_libc_test_lib::{call, call_show};

// added:
macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
}

macro_rules! recurrence {
    (  $seq:ident [ $ind:ident ]: $sty:ty = $($inits:expr),+ => $recur:expr ) => {
        {
            use std::ops::Index;

            const MEM_SIZE: usize = count_exprs!($($inits),+);

            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }

            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }

            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;

                #[inline(always)]
                fn index<'b>(&'b self, index: usize) -> &'b $sty {
                    use std::num::Wrapping;

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);
                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }

            impl Iterator for Recurrence {
                type Item = $sty;

                #[inline]
                fn next(&mut self) -> Option<$sty> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $ind = self.pos;
                            let $seq = IndexOffset { slice: &self.mem, offset: $ind };
                            $recur
//                          ^~~~~~ changed
                        };
                        {
                            use std::mem::swap;
                            let mut swap_tmp = next_val;
                            for i in (0..MEM_SIZE).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }
                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recurrence { mem: [$($inits),+], pos: 0 }
        }
    };
}
fn main() {
    call_show();
    println!("{}", call(12) as i32);
    println!("Hello, world!");
    // let fib = recurrence![a[n] = 0, 1, ..., a[n-1] + a[n-2]];
    let fib = recurrence![a[n]: u64 = 0, 1, 2 => a[n-2] + a[n-1] + a[n-2]];

    for e in fib.take(10) { println!("{}", e) }

    let fib2 = recurrence!(a[n]: u64 = 1, 2 => a[n-1] + a[n-2]);

    for e in fib2.take(10) { println!("{}", e) }

    let fib2 = recurrence!(a[n]: u32 = 1, 2 => a[n-1] + a[n-2]);

    for e in fib2.take(10) { println!("{}", e) }


    let a = {
        #[derive(Debug)]
        struct A<T>(T);
        A(1)
    };

    println!("{:?}", a);
}
