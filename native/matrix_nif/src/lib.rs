#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

use rustler::{Env, Term, NifResult, Encoder, Error};
use rustler::types::list::ListIterator;

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.MatrixNif",
    [("transpose", 1, transpose)],
    None
}

fn transpose<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let r1: NifResult<Vec<Vec<i64>>> = args[0].decode();
    match r1 {
        Err(e) => {
            let m1: Vec<Vec<f64>> = args[0].decode()?;
            let mut m2 = vec![vec![0.0; m1.len()]; m1[0].len()];
            (0..(m1.len()))
                .for_each(|r| {
                    (0..(m1[r].len())).for_each(|c| {
                        m2[c][r] = m1[r][c]
                    })
                });
            Ok(m2.encode(env))
        },
        Ok(m1) => {
            let mut m2 = vec![vec![0; m1.len()]; m1[0].len()];
            (0..(m1.len()))
                .for_each(|r| {
                    (0..(m1[r].len())).for_each(|c| {
                        m2[c][r] = m1[r][c]
                    })
                });
            Ok(m2.encode(env))
        },
    }
}
