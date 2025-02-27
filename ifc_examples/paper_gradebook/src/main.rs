use secret_macros::untrusted_secure_block_dynamic_all;
use secret_structs::secret::{*, DynLabel, Int, Sec, SecureValue};

use crate::grades::grades::new_grade_vec;

mod grades;

fn main() {
    //Fig. 1
    let grades: Vec<i32> = vec![90, 80, 70, 60, 50];
    let avg = grades::grades::compute_average(&grades);
    println!("Average: {}", avg);

    //Fig. 7
    let grades: Vec<SecureValue<i32, grades::grades::TeacherPub, grades::grades::TeacherEndorsed, DynLabel<Sec>, DynLabel<Int>>> = new_grade_vec();
    let grades_dyn_sec = grades[0].get_dyn_sec_label_ref().join(grades[1].get_dyn_sec_label_ref()).join(grades[2].get_dyn_sec_label_ref()).join(grades[3].get_dyn_sec_label_ref()).join(grades[4].get_dyn_sec_label_ref());
    let grades_dyn_int = grades[0].get_dyn_int_label_ref().join(grades[1].get_dyn_int_label_ref()).join(grades[2].get_dyn_int_label_ref()).join(grades[3].get_dyn_int_label_ref()).join(grades[4].get_dyn_int_label_ref());
    #[allow(unused_variables)]
    let declassified_avg: SecureValue<i32, grades::grades::TeacherPub, grades::grades::TeacherEndorsed, DynLabel<Sec>, DynLabel<Int>> = 
        grades::grades::compute_and_declassify_average(&grades, grades_dyn_sec, grades_dyn_int, DynLabel::<Sec>::new_default());

    let label_s = DynLabel::<Sec>::new_size_one(get_new_secrecy_tag());
    let label_i = DynLabel::<Int>::new_size_one(get_new_integrity_tag());
    let label_s2 = DynLabel::<Sec>::new_size_one(get_new_secrecy_tag());
    let label_i2 = DynLabel::<Int>::new_size_one(get_new_integrity_tag());
    let grade = untrusted_secure_block_dynamic_all!(grades::grades::TeacherPub, grades::grades::TeacherEndorsed, &label_s, &label_i, {
        wrap(90)
    });
    let g2 = untrusted_secure_block_dynamic_all!(grades::grades::TeacherPub, grades::grades::TeacherEndorsed, &label_s.join(&label_s2), &label_i2, {
        grade
    });

}