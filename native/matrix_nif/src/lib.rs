#[macro_use] extern crate rustler;
//#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;

use rustler::{Env, Term, NifResult, Encoder};
use rustler::env::{OwnedEnv, SavedTerm};

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
    let pid = env.pid();
    let mut my_env = OwnedEnv::new();

    let saved_list = my_env.run(|env| -> NifResult<SavedTerm> {
        let list_arg = args[0].in_env(env);
        Ok(my_env.save(list_arg))
    })?;

    std::thread::spawn(move || {
        my_env.send_and_clear(&pid, |env| {
            let result: NifResult<Term> = (|| {
                let reserved_list = saved_list.load(env);
                let r1: NifResult<Vec<Vec<i64>>> = reserved_list.decode();
                match r1 {
                    Err(_e) => {
                        let m1: Vec<Vec<f64>> = reserved_list.decode()?;
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
            })();
            match result {
                Err(_err) => env.error_tuple("test failed".encode(env)),
                Ok(term) => term
            }
        });
    });

    Ok(atoms::ok().to_term(env))
}
