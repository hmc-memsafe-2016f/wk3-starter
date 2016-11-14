extern crate wk3;

pub use wk3::{DB,DBView,filter_one,filter_two};


// This macro is an assertion with nicely formatted failure output
macro_rules! assert_expected_eq_actual {
    ($a:expr, $b:expr) => ({
        let (a, b) = (&$a, &$b);
        assert!(*a == *b,
                "\nExpected `{:?}` is not equal to Actual `{:?}`\nAssertion: `assert_expected_eq_actual!({}, {})`",
                *a,
                *b,
                stringify!($a),
                stringify!($b));
    })
}

mod iterator_bonus {
    mod filter_one {
        use super::super::{DB,filter_one};

        fn always_true(_: &i32) -> bool { true }

        fn is_positive(i: &i32) -> bool { *i > 0 }

        #[test]
        #[allow(unused_variables)]
        fn simple() {
            let a = DB::new(vec![5]);
            let view_a = a.select_where(&always_true);
            let view_a_2 = filter_one(&view_a, is_positive);
        }

        #[test]
        #[allow(unused_variables)]
        fn underlying_data() {
            let a = DB::new(vec![5]);
            let view_a_2 = {
                let view_a = a.select_where(&always_true);
                filter_one(&view_a, is_positive)
            };
        }
    }

    mod filter_two {
        use super::super::{DB,filter_two};

        fn always_true(_: &i32) -> bool { true }

        fn is_positive(i: &i32) -> bool { *i > 0 }

        #[test]
        #[allow(unused_variables)]
        fn simple() {
            let a = DB::new(vec![5]);
            let view_a = a.select_where(&always_true);
            let b = DB::new(vec![6]);
            let view_b = b.select_where(&always_true);
            let (view_a_2, view_b_2) = filter_two(&view_a, &view_b, &always_true);
        }

        #[test]
        fn complex() {
            let a = DB::new(vec![-1, 5, 0]);
            let view_a = a.select_where(&is_positive);
            let b = DB::new(vec![6, -4, 3]);
            let view_b = b.select_where(&is_positive);
            let (view_a_2, view_b_2) = filter_two(&view_a, &view_b, &always_true);
            assert_expected_eq_actual!(vec![5],
                                       view_a_2.into_iter().cloned().collect::<Vec<_>>());
            assert_expected_eq_actual!(vec![6, 3],
                                       view_b_2.into_iter().cloned().collect::<Vec<_>>());
        }

        #[test]
        fn check_lifetimes() {
            let a = DB::new(vec![0, 5, 0, 0]);
            let view_a_2 =
            {
                let b = DB::new(vec![6, -5, -7, 3, -1]);
                let view_b = b.select_where(&is_positive);
                let view_a_2 = {
                    let view_a = a.select_where(&is_positive);
                    let (view_a_2, view_b_2) = filter_two(&view_a, &view_b, &always_true);
                    assert_expected_eq_actual!(vec![6, 3],
                                               view_b_2.into_iter().cloned().collect::<Vec<_>>());
                    view_a_2
                };
                view_a_2
            };
            assert_expected_eq_actual!(vec![5],
                                       view_a_2.into_iter().cloned().collect::<Vec<_>>());
        }

        #[test]
        fn check_lifetimes_reverse() {
            let a = DB::new(vec![0, 5, 0, 0]);
            let view_a = a.select_where(&is_positive);
            let view_a_2 =
            {
                let b = DB::new(vec![6, -5, -7, 3, -1]);
                let view_b = b.select_where(&is_positive);
                let (view_b_2, view_a_2) = filter_two(&view_b, &view_a, &always_true);
                assert_expected_eq_actual!(vec![6, 3],
                                           view_b_2.into_iter().cloned().collect::<Vec<_>>());
                view_a_2
            };
            assert_expected_eq_actual!(vec![5],
                                       view_a_2.into_iter().cloned().collect::<Vec<_>>());
        }
    }

    mod db_select {
        use super::super::DB;

        #[derive(Clone, Debug, PartialEq, Eq)]
        struct NoCopy(i32);

        fn is_positive(i: &NoCopy) -> bool { i.0 > 0 }

        fn is_even(i: &NoCopy) -> bool { i.0 % 2 == 0 }

        #[test]
        fn just_construct() {
            let v: Vec<_> = (5..6).map(NoCopy).collect();
            let x = DB::new(v.clone());
            assert_expected_eq_actual!(v.len(), x.len());
        }

        #[test]
        fn just_construct_large() {
            let v: Vec<_> = (0..100).map(NoCopy).collect();
            let x = DB::new(v.clone());
            assert_expected_eq_actual!(v.len(), x.len());
        }

        #[test]
        fn construct_and_check() {
            let v: Vec<_> = (5..6).map(NoCopy).collect();
            let x = DB::new(v.clone());
            assert_expected_eq_actual!(v.len(), x.len());
            assert_expected_eq_actual!(v, x.into_iter().collect::<Vec<_>>());
        }

        #[test]
        fn construct_and_check_large() {
            let v: Vec<_> = (0..6).map(NoCopy).collect();
            let x = DB::new(v.clone());
            assert_expected_eq_actual!(v.len(), x.len());
            assert_expected_eq_actual!(v, x.into_iter().collect::<Vec<_>>());
        }

        #[test]
        fn construct_select_and_check() {
            let v: Vec<_> = (5..6).map(NoCopy).collect();
            let filter = is_even;
            let x = DB::new(v.clone());
            let x_view = x.select_where(filter);
            let v_filtered = v.into_iter().filter(filter).collect::<Vec<_>>();
            assert_expected_eq_actual!(v_filtered.len(), x_view.len());
            assert_expected_eq_actual!(v_filtered, x_view.into_iter().cloned().collect::<Vec<_>>());
        }

        #[test]
        fn construct_select_and_check_large() {
            let v: Vec<_> = (-100..100).map(NoCopy).collect();
            let filter = is_positive;
            let x = DB::new(v.clone());
            let x_view = x.select_where(filter);
            let v_filtered = v.into_iter().filter(filter).collect::<Vec<_>>();
            assert_expected_eq_actual!(v_filtered.len(), x_view.len());
            assert_expected_eq_actual!(v_filtered, x_view.into_iter().cloned().collect::<Vec<_>>());
        }
    }

    mod db_view_select {
        use super::super::DB;

        #[derive(Clone, Debug, PartialEq, Eq)]
        struct NoCopy(i32);

        fn always_true(_: &NoCopy) -> bool { true }

        fn is_positive(i: &NoCopy) -> bool { i.0 > 0 }

        fn is_even(i: &NoCopy) -> bool { i.0 % 2 == 0 }

        #[test]
        fn just_construct() {
            let v: Vec<_> = (-100..100).map(NoCopy).collect();
            let x = DB::new(v.clone());
            let x_view = x.as_view();
            let x_view_2 = x.select_where(always_true);
            assert_expected_eq_actual!(v.len(), x_view.len());
            assert_eq!(x_view, x_view_2);
        }

        #[test]
        fn construct_and_select() {
            let v: Vec<_> = (-100..100).map(NoCopy).collect();
            let filter = is_even;
            let filter2 = is_positive;
            let x = DB::new(v.clone());
            let x_view = x.select_where(filter).select_where(filter2);
            let v_filtered = v.into_iter().filter(filter).filter(filter2).collect::<Vec<_>>();
            assert_expected_eq_actual!(v_filtered.len(), x_view.len());
            assert_expected_eq_actual!(v_filtered, x_view.into_iter().cloned().collect::<Vec<_>>());
        }

        #[test]
        fn check_lifetimes() {
            let v: Vec<_> = (-100..100).map(NoCopy).collect();
            let filter = is_even;
            let filter2 = is_positive;
            let x = DB::new(v.clone());
            let x_view_2 = {
                let x_view = x.select_where(filter);
                x_view.select_where(filter2)
            };
            let v_filtered = v.into_iter().filter(filter).filter(filter2).collect::<Vec<_>>();
            assert_expected_eq_actual!(v_filtered.len(), x_view_2.len());
            assert_expected_eq_actual!(v_filtered, x_view_2.into_iter().cloned().collect::<Vec<_>>());
        }

    }

    mod db_select_mut {
        use super::super::DB;

        #[derive(Clone, Debug, PartialEq, Eq)]
        struct NoCopy(i32);

        fn always_true(_: &NoCopy) -> bool { true }

        fn is_positive(i: &NoCopy) -> bool { i.0 > 0 }

        fn is_even(i: &NoCopy) -> bool { i.0 % 2 == 0 }

        #[test]
        fn just_construct() {
            let v: Vec<_> = (-100..100).map(NoCopy).collect();
            let mut x = DB::new(v.clone());
            let x_view = x.as_view_mut();
            assert_expected_eq_actual!(v.len(), x_view.len());
        }

        #[test]
        fn just_construct_as_view_mut() {
            let v: Vec<_> = (-100..100).map(NoCopy).collect();
            let mut x = DB::new(v.clone());
            let x_view = x.select_where_mut(always_true);
            assert_expected_eq_actual!(v.len(), x_view.len());
        }

        #[test]
        fn construct_and_select() {
            let mut v: Vec<_> = (-100..100).map(NoCopy).collect();
            let filter = is_even;
            let filter2 = is_positive;
            let mut x = DB::new(v.clone());
            v.retain(is_even);
            v.retain(is_positive);
            let x_view = x.select_where_mut(filter).select_where_mut(filter2);
            let v_filtered = v.iter_mut().collect::<Vec<_>>();
            assert_expected_eq_actual!(v_filtered.len(), x_view.len());
            assert_expected_eq_actual!(v_filtered, x_view.into_iter().collect::<Vec<_>>());
        }

        #[test]
        fn construct_select_and_modify() {
            let mut v: Vec<_> = (-100..100).map(NoCopy).collect();
            let filter = is_even;
            let filter2 = is_positive;
            let mut x = DB::new(v.clone());
            for &mut NoCopy(ref mut x) in v.iter_mut() {
                if filter(&NoCopy(*x)) && filter2(&NoCopy(*x)) {
                    *x += 1;
                }
            }
            for &mut NoCopy(ref mut x) in x.select_where_mut(filter).select_where_mut(filter2) {
                *x += 1;
            }
            assert_expected_eq_actual!(v.len(), x.len());
            assert_expected_eq_actual!(v, x.into_iter().collect::<Vec<_>>());
        }
    }
}
