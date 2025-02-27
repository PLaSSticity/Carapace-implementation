#![feature(trace_macros, negative_impls)]
use secret_macros::untrusted_secure_block_dynamic_all;
#[allow(unused_imports)]
use secret_structs::secret::{self as sec,Secret,DynamicTag,*};  
use secret_structs::ternary_lattice as sec_lat;
use secret_structs::integrity_lattice as int_lat;
mod grades;
extern crate libc;
use std::time::Instant;

#[allow(dead_code)]
fn synthetic_benchmarks() {
    println!("====================================");
    let dummystag = sec::DynLabel::<Sec>::new_size_one(sec::get_new_secrecy_tag());
    let dummyitag = sec::DynLabel::<Int>::new_default();
    let dummyval = untrusted_secure_block_dynamic_all!(sec_lat::Label_A, int_lat::Label_All, &dummystag, &dummyitag, {
        wrap(0)
    });
    println!("Timing 20m label clones...");
    let start = Instant::now();
    for _ in 0..20000000 {
        dummyval.get_dyn_sec_label();
    }
    println!("Result {:?}", Instant::now().duration_since(start));
    //secret_structs::secret::print_counters_c();
    println!("====================================");
    println!("Timing 20m small label subset tests...");
    let dummystag = sec::DynLabel::<Sec>::new_size_one(sec::get_new_secrecy_tag());
    let start = Instant::now();
    for _ in 0..20000000 {
        unsafe { dummystag.subset_of(&dummystag) };
    }
    println!("Result {:?}", Instant::now().duration_since(start));
    //secret_structs::secret::print_counters_c();
    println!("====================================");
    println!("Timing 50k large label op tests...");
    //simulates an average calculation?
    let mut accumlabel = sec::DynLabel::<Sec>::new_default();
    let start = Instant::now();
    for _ in 0..50000 {
        let dummystag = sec::DynLabel::<Sec>::new_size_one(sec::get_new_secrecy_tag());
        accumlabel = accumlabel.join(&dummystag);
        unsafe { dummystag.subset_of(&accumlabel) };
    }
    println!("Result {:?}", Instant::now().duration_since(start));
    println!("====================================");
}

fn main() {
    secret_structs::secret::initialize_carapace();
    let args = std::env::args().collect::<Vec<String>>();
    if args[1] == "ifc_add" {
        //println!("Ifc Add");
        let mut sec_tags: Vec<DynamicTag<Sec>> = vec![];
        let mut int_tags: Vec<DynamicTag<Int>> = vec![];
        /*let sec_t = sec::get_new_secrecy_tag();
        let int_t = sec::get_new_integrity_tag();
        for i in 0..1000 {
            sec_tags.push(sec_t.clone());
            int_tags.push(int_t.clone());
        }*/
        for _i in 0..1000 {
            sec_tags.push(sec::get_new_secrecy_tag());
            int_tags.push(sec::get_new_integrity_tag());
        }
        let mut gs: grades::grades::GradeSet<sec_lat::Label_AB, int_lat::Label_NotXYZ> = grades::grades::initialize(&sec_tags, &int_tags);
        let mut rng = rand::thread_rng();
        let start = before_timed_experiment();
        do_ifc_add(&mut rng, &mut gs);
        after_timed_experiment("Ifc", start);
        //let label = gs.grades[0][0].get_dyn_sec_label();
        //get_hash(label);
    } else if args[1] == "non_ifc_add" {
        //println!("Non Ifc Add");
        let mut gs: grades::grades::PublicGradeSet = grades::grades::initialize_public(1000);
        let mut rng = rand::thread_rng();
        let start = before_timed_experiment();
        do_non_ifc_add(&mut rng, &mut gs);
        after_timed_experiment("Non-ifc", start);
    } else if args[1] == "ifc_average" {
        //println!("Ifc Average");
        //Average runs
        let mut sec_tags: Vec<DynamicTag<Sec>> = vec![];
        let mut int_tags: Vec<DynamicTag<Int>> = vec![];
        for _i in 0..1000 { 
            sec_tags.push(sec::get_new_secrecy_tag());
            int_tags.push(sec::get_new_integrity_tag());
        }
        let mut gs: grades::grades::GradeSet<sec_lat::Label_AB, int_lat::Label_NotXYZ> = grades::grades::initialize(&sec_tags, &int_tags);
        let mut average_secret_label = secret_structs::secret::DynLabel::<Sec>::new_default();
        let mut average_integrity_label = secret_structs::secret::DynLabel::<Int>::new_default();
        for i in 0..1000 {
            average_secret_label = average_secret_label.join(&secret_structs::secret::DynLabel::<Sec>::new_size_one(sec_tags[i].clone()));
            average_integrity_label = average_integrity_label.join(&secret_structs::secret::DynLabel::<Int>::new_size_one(int_tags[i].clone()));
        }
        let start = before_timed_experiment();
        let accumulator = do_ifc_average(&mut gs, &average_secret_label, &average_integrity_label);
        after_timed_experiment("Ifc", start);
        std::hint::black_box(accumulator);
    } else if args[1] == "non_ifc_average" {
        //println!("Non Ifc Average");
        let mut gs: grades::grades::PublicGradeSet = grades::grades::initialize_public(1000);
        let mut rng = rand::thread_rng();
        let start = before_timed_experiment();
        let accumulator = do_non_ifc_average(&mut gs);
        after_timed_experiment("Non-ifc", start);
        std::hint::black_box(accumulator);
    } else {
        println!("Other");
    }
}

#[inline(never)]
fn do_ifc_add(_rng: &mut rand::rngs::ThreadRng, gs: &mut grades::grades::GradeSet<sec_lat::Label_AB, int_lat::Label_NotXYZ>) {
  for i in 0..10000000 {
    let j = (i * 17) % 1000;
    gs.add_to_grade(i, j as usize);
  }
  /*
  for _i in 0..10000 {
    let r = rng.gen_range(0..10);
    for j in 0..1000 {
        gs.add_to_grade(r, j);
    }
  }
  */
}

#[inline(never)]
fn do_non_ifc_add(_rng: &mut rand::rngs::ThreadRng, gs: &mut grades::grades::PublicGradeSet) {
  for i in 0..10000000 {
    let j = (i * 17) % 1000;
    gs.add_to_grade(i, j as usize);
  }
  /*
  for _i in 0..10000 {
    let r = rng.gen_range(0..10);
    for j in 0..1000 {
        gs.add_to_grade(r, j);
    }
  }
  */
}

#[inline(never)]
fn do_ifc_average(gs: &mut grades::grades::GradeSet<sec_lat::Label_AB, int_lat::Label_NotXYZ>, average_secret_label: &DynLabel<Sec>, average_integrity_label: &DynLabel<Int>)-> i32 {
  let mut accumulator = 0;
  for _i in 0..10000 {
    accumulator += gs.benchmark_average(average_secret_label, average_integrity_label);
  }
  accumulator
}

#[inline(never)]
fn do_non_ifc_average(gs: &mut grades::grades::PublicGradeSet) -> i32 {
  let mut accumulator = 0;
  for _i in 0..10000 {
    accumulator += gs.benchmark_average();
  }
  accumulator
}

fn before_timed_experiment() -> Instant {
  /*
  eprintln!("Counters BEFORE timed experiment:");
  unsafe { secret_structs::secret::print_counters(); }
  */
  Instant::now()
}

fn after_timed_experiment(msg: &str, start: Instant) {
  /*
  eprintln!("Counters AFTER timed experiment:");
  unsafe { secret_structs::secret::print_counters(); }
  */
  let elapsed = start.elapsed();
  println!("{} Elapsed: {:.2?}µs", msg, elapsed.as_micros());
}
