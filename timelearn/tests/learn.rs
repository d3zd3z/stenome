// Test the learning code.

extern crate rand;
extern crate tempdir;
extern crate timelearn;

use rand::{Rng, SeedableRng, XorShiftRng};
use tempdir::TempDir;
use timelearn::{now, Populator, Store, Result};
use std::io::Write;

#[test]
fn learning() {
    let tmp_dir = TempDir::new("learn").unwrap();
    let db_path = tmp_dir.path().join("learn.db");

    {
        let mut st = Store::create(&db_path, "test").unwrap();
        assert_eq!(st.get_kind(), "test");

        populate(&mut st).unwrap();

        // Ask all of the problems that we can.
        let mut num = 1;
        loop {
            let prob = match st.get_next().unwrap() {
                None => break,
                Some(p) => p,
            };

            println!("Ask {} ({})", prob.question, num);
            st.update(prob, num).unwrap();
            num = num % 4 + 1;
        }

        // Then ask all of the new problems.
        loop {
            let prob = match st.get_new().unwrap() {
                None => break,
                Some(p) => p,
            };

            println!("New {} ({})", prob.question, num);
            st.update(prob, num).unwrap();
            num = num % 4 + 1;
        }

        // The challenge here then is figuring out if this result is meaningful.
    }

    // Close and open to make sure that works.
    let st = Store::open(&db_path).unwrap();
    assert_eq!(st.get_kind(), "test");
}

// Populate with test data.  Make a mix of unlearned problems, and learned ones that are ready to
// learn, and those that aren't ready to learn.
fn populate(st: &mut Store) -> Result<()> {
    let mut p = st.populate()?;
    let cur = now();

    // These are in the past, ready to ask.
    for i in 1..11 {
        add_one(&mut p, i, Some((cur - 20.0, 5.0)))?;
    }

    // These are in the future.
    for i in 11..21 {
        add_one(&mut p, i, Some((cur + 20.0, 5.0)))?;
    }

    // And these haven't been asked at all.
    for i in 21..31 {
        add_one(&mut p, i, None)?;
    }

    p.commit()?;
    Ok(())
}

// Add a problem of the given number.  Next and interval can optionally be set.
fn add_one<'a>(pop: &mut Populator<'a>, num: i32, ni: Option<(f64, f64)>) -> Result<()> {
    let mut rng: XorShiftRng = SeedableRng::from_seed([num as u32, 1, 2, 3]);

    let mut qn = vec![];
    write!(&mut qn, "qn{}:", num).unwrap();
    for _ in 1..rng.gen_range(5, 40) {
        qn.push(rng.gen_range(b'a', b'z'));
    }

    let mut an = vec![];
    write!(&mut an, "an{}:", num).unwrap();
    for _ in 1..rng.gen_range(5, 40) {
        an.push(rng.gen_range(b'a', b'z'));
    }

    let qn = String::from_utf8(qn).unwrap();
    let an = String::from_utf8(an).unwrap();
    match ni {
        None => pop.add_problem(&qn, &an)?,
        Some((next, interval)) => pop.add_learning_problem(&qn, &an, next, interval)?,
    }

    Ok(())
}
