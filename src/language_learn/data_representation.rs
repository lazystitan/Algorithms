struct DataA {
    pub a : i32,
    pub b : u64,
    pub c : bool
}

/*
    &DataA 0xdf352ff560
    &a 0xdf352ff560
    &b 0xdf352ff560
    &c 0xdf352ff560
    b 8byte 0xdf352ff560
                       1
                       2
                       3
                       4
                       5
                       6
                       7
    a 4byte 0xdf352ff568
                       9
                       a
                       b
    c       0xdf352ff56c

*/

struct DataB {
    pub a : i32,
    pub b : i32,
    pub c : ()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn data_a_test() {
        let d = DataA {
            a : 32,
            b : 64,
            c : true
        };

        let ad = &d as *const _;
        println!("{:?}",ad);
        let ad_a = &d.a as *const _;
        let ad_b = &d.b as *const _;
        let ad_c = &d.c as *const _;
        println!("{:?}-{:?}-{:?}", ad_a, ad_b, ad_c);
        assert!(false);
    }

    #[test]
    fn multiple_data_a_test() {
        let d1 = DataA {
            a : 32,
            b : 64,
            c : true
        };

        let d2 = DataA {
            a : 32,
            b : 64,
            c : true
        };

        let d3 = DataA {
            a : 32,
            b : 64,
            c : true
        };

        let ad1 = &d1 as *const _;
        let ad2 = &d2 as *const _;
        let ad3 = &d3 as *const _;
        println!("{:?}-{:?}-{:?}",ad1, ad2, ad3);
        let ad_a = &d1.a as *const _;
        let ad_b = &d1.b as *const _;
        let ad_c = &d1.c as *const _;
        println!("{:?}-{:?}-{:?}", ad_a, ad_b, ad_c);
        let ad_a = &d2.a as *const _;
        let ad_b = &d2.b as *const _;
        let ad_c = &d2.c as *const _;
        println!("{:?}-{:?}-{:?}", ad_a, ad_b, ad_c);
        let ad_a = &d3.a as *const _;
        let ad_b = &d3.b as *const _;
        let ad_c = &d3.c as *const _;
        println!("{:?}-{:?}-{:?}", ad_a, ad_b, ad_c);
        assert!(false);
    }

    /*
    &d1 0x4e4e0ff2b0
    &a 0x4e4e0ff2b8
    &b 0x4e4e0ff2b0
    &c 0x4e4e0ff2bc

    0x4e4e0ff2b0
               1
               2
               3
               4
               5
               6
               7
    0x4e4e0ff2b8
               9
               a
               b
    0x4e4e0ff2bc
               d
               e
               f
    0x4e4e0ff2c0

    0x4e4e0ff2c0 - 0x4e4e0ff2b0 = 0x10 = 16byte

    &d2 0x4e4e0ff2c0
    &a 0x4e4e0ff2c8
    &b 0x4e4e0ff2c0
    &c 0x4e4e0ff2cc

    0x4e4e0ff2d0 - 0x4e4e0ff2c0 = 0x10 = 16byte

    &d3 0x4e4e0ff2d0
    &a 0x4e4e0ff2d8
    &b 0x4e4e0ff2d0
    &c 0x4e4e0ff2dc

    */

    #[test]
    fn zero_size_type_test() {
        let a = 10;
        let empty = ();
        let b = 10;

        let aa = &a as *const _;
        let az = &empty as *const _;
        let ab = &b as *const _;

        println!("{:?}-{:?}-{:?}", aa, az, ab);
        assert!(false);
    }

    /*
    aa 0x7fd28ff59c
                  d
                  e
                  f
    az 0x7fd28ff5a0
                  1
                  2
                  3
    ab 0x7fd28ff5a4
    */

    #[test]
    fn sizeof_test() {
        let a = size_of::<()>();
        let b = size_of::<DataA>();
        let c = size_of::<DataB>();
        println!("{}", a);
        println!("{}", b);
        println!("{}", c);
        assert!(false);
    }

    #[test]
    fn multiple_data_b_test() {
        let d1 = DataB {
            a : 32,
            b : 64,
            c : ()
        };

        let d2 = DataB {
            a : 32,
            b : 64,
            c : ()
        };

        let d3 = DataB {
            a : 32,
            b : 64,
            c : ()
        };

        let ad1 = &d1 as *const _;
        let ad2 = &d2 as *const _;
        let ad3 = &d3 as *const _;
        println!("{:?}-{:?}-{:?}",ad1, ad2, ad3);
        let ad_a = &d1.a as *const _;
        let ad_b = &d1.b as *const _;
        let ad_c = &d1.c as *const _;
        println!("{:?}-{:?}-{:?}", ad_a, ad_b, ad_c);
        let ad_a = &d2.a as *const _;
        let ad_b = &d2.b as *const _;
        let ad_c = &d2.c as *const _;
        println!("{:?}-{:?}-{:?}", ad_a, ad_b, ad_c);
        let ad_a = &d3.a as *const _;
        let ad_b = &d3.b as *const _;
        let ad_c = &d3.c as *const _;
        println!("{:?}-{:?}-{:?}", ad_a, ad_b, ad_c);
        assert!(false);
    }

    /*
    0x8d692ff150
    0x8d692ff160
    0x8d692ff170
    */

    #[test]
    fn address_test() {
        let a = 12;
        let d = 1 as u64 as i64;
        let b = &a as *const i32;
        let c = &d as *const i64;
        println!("{:?}", b);
        println!("{:?}", c);
        assert!(false);
    }

}