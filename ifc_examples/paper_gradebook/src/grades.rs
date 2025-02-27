pub mod grades {
    use secret_structs::{*, secret::*, ternary_lattice::*};
    //Infrastucture for Fig. 7
    pub fn new_grade_vec() -> Vec<SecureValue<i32, TeacherPub, TeacherEndorsed, DynLabel<Sec>, DynLabel<Int>>> {
        let mut v = vec![];
        for i in 0..5 {
            let s_label = DynLabel::<Sec>::new_size_one(get_new_secrecy_tag());
            let i_label = DynLabel::<Int>::new_size_one(get_new_integrity_tag());
            v.push(untrusted_secure_block_dynamic_all!(TeacherPub, TeacherEndorsed, &s_label, &i_label, {
                wrap(90 - i * 10)
            }));
        }
        v
    }

    //Fig. 1
    pub fn compute_average(grades: &Vec<i32>) -> i32 {
        let mut total = 0;
        for grade in grades {
            total += grade;
        }
        total / grades.len() as i32
    }


    //Fig. 4
    pub type TeacherSec = secret_structs::ternary_lattice::Label_A;
    pub type TeacherPub = secret_structs::ternary_lattice::Label_Empty;
    pub type TeacherEndorsed = secret_structs::integrity_lattice::Label_All;
    #[allow(dead_code)]
    pub type TeacherUnendorsed = secret_structs::integrity_lattice::Label_NotX;

    //Fig. 5
    //CHANGE: Needs a bound on GradeSec making it more secret than TeacherSec
    #[allow(dead_code)]
    pub fn add_bonus_to_grade<GradeSec: MoreSecretThan<Label_Empty> + MoreSecretThan<TeacherSec>>(
        grade: &mut SecureValue<i32, GradeSec, TeacherEndorsed, DynLabel<Sec>, DynLabel<Int>>,
        bonus: &SecureValue<i32, TeacherSec, TeacherEndorsed, DynLabel<Sec>, DynLabel<Int>>,
    ) {
        untrusted_secure_block_no_return_dynamic_all!(GradeSec, TeacherEndorsed, 
                                            grade.get_dyn_sec_label_ref(), grade.get_dyn_int_label_ref(),{
            let g = unwrap_mut_ref(grade);
            let b = unwrap_ref(bonus);
            *g += *b;
        });
    }

    //Fig. 6
    //For some reason, GradeSec needs to be defined as MoreSecretThan<GradeSec> or it doesn't work.
    #[allow(dead_code)]
    pub fn declassify_and_or_endorse_grade<GradeSec: MoreSecretThan<GradeSec>, OutSec: MoreSecretThan<Label_Empty>>(
        grade: &SecureValue<i32, GradeSec, TeacherEndorsed, DynLabel<Sec>, DynLabel<Int>>,
        out_sec: DynLabel<Sec>, out_int: DynLabel<Int>
    ) -> SecureValue<i32, OutSec, TeacherEndorsed, DynLabel<Sec>, DynLabel<Int>> {
        partial_trusted_secure_block_dynamic_all!(GradeSec, TeacherEndorsed, 
                                        grade.get_dyn_sec_label_ref(), grade.get_dyn_int_label_ref(),
                                        OutSec, TeacherEndorsed,
                                        &out_sec, &out_int, {
            wrap(*unwrap_ref(grade))
        })
    }

    //Fig. 7
    //CHANGE: Replace count from paper with std::vec::Vec::len
    //For some reason, GradeSec needs to be defined as MoreSecretThan<GradeSec> or it doens't work.
    pub fn compute_and_declassify_average<GradeSec: Label>(
        student_grade_vec: &Vec<SecureValue<i32, GradeSec, TeacherEndorsed, DynLabel<Sec>, DynLabel<Int>>>,
        blk_sec: DynLabel<Sec>, blk_int: DynLabel<Int>,
        out_sec: DynLabel<Sec>
    ) -> SecureValue<i32, TeacherPub, TeacherEndorsed, DynLabel<Sec>, DynLabel<Int>> {
        let classified_average = untrusted_secure_block_dynamic_all!(GradeSec, TeacherEndorsed, 
                                                                                                        &blk_sec, &blk_int, {
            let mut total = 0;
            for secure_grade in student_grade_vec {
                total += *unwrap_ref(secure_grade);
            }
            wrap(total / std::vec::Vec::len(student_grade_vec) as i32)
        });
        partial_trusted_secure_block_dynamic_all!(GradeSec, TeacherEndorsed, &blk_sec, &blk_int,
                                        TeacherPub, TeacherEndorsed, &out_sec, &blk_int, {
            wrap(unwrap(classified_average))
        })
    }
}