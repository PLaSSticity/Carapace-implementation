#[allow(unused_imports)]

//trace_macros!(true);
pub mod grades {
    use std::ops::Add;
    use secret_structs::secret::{self as sec,Secret,DynamicTag,*};  
    use secret_structs::ternary_lattice as sec_lat;
    use secret_structs::integrity_lattice as int_lat;
    use secret_macros::{self, *};
    use std::fs;
    #[derive(InvisibleSideEffectFreeDerive)]
    pub struct GradeSet<L1, L2> where L1: sec_lat::MoreSecretThan<L1> + secret_structs::ternary_lattice::Label, L2: int_lat::LowerIntegrityThan<L2> + secret_structs::ternary_lattice::Label
    {
        pub grades: Vec<(SecureValue<i32, L1, L2, DynLabel<Sec>, DynLabel<Int>>,String)> // string is student name
        //pub grades: Vec<Vec<SecureValue<i32, L1, L2, DynLabel<Sec>, DynLabel<Int>>>>
        //grades: [[SecureValue<i32, L1, L2, DynamicSecretLabel, DynamicIntegrityLabel>; 4]; 4]
    }

    pub struct PublicGradeSet
    {
        grades: Vec<(i32,String)> // the string is student name
    }

    pub fn initialize_public(length: usize) -> PublicGradeSet {
        let mut grades = vec![];
        for i in 0..length {
            grades.push((i as i32,String::from("")));
        }
        PublicGradeSet{grades: grades}
    }

    impl PublicGradeSet {
        pub fn add_to_grade(&mut self, added: i32, student_id: usize) {
            let student_grades = &mut self.grades;
            let secret = &mut student_grades[student_id];
            secret.0 += added;
        }

        pub fn benchmark_average(&self) -> i32 {
            let student_grades = &self.grades;
            let mut average = 0;
            let count = self.grades.len();
            for i in 0..self.grades.len() {
                let secret = &student_grades[i];
                average += secret.0;
            }
            average /= count as i32;
            average
        }
    }

    pub fn initialize<L1, L2>(sec_tags: &Vec<DynamicTag<Sec>>, int_tags: &Vec<DynamicTag<Int>>) -> GradeSet<L1, L2>
    where L1: secret_structs::ternary_lattice::Label, L2: secret_structs::ternary_lattice::Label {
        //let tags = vec![1, 2, 3, 4, 23, 24, 25, 26];
        let mut grades = vec![];
        for i in 0..sec_tags.len() {
            //grades.push(vec![]);
            //for j in 0..4 {
                //grades[i].push(
                grades.push(
                    (untrusted_secure_block_dynamic_all!(L1, L2, &sec::DynLabel::<Sec>::new_size_one(sec_tags[i].clone()), &sec::DynLabel::<Int>::new_size_one(int_tags[i].clone()), {
                        wrap(i as i32)
                    }),String::from(""))
                );
            //}
        }
        GradeSet{grades: grades}
        //GradeSet{grades: [[grades[0][0].clone(), grades[0][1].clone(), grades[0][2].clone(), grades[0][3].clone()], [grades[1][0].clone(), grades[1][1].clone(), grades[1][2].clone(), grades[1][3].clone()], [grades[2][0].clone(), grades[2][1].clone(), grades[2][2].clone(), grades[2][3].clone()], [grades[3][0].clone(), grades[3][1].clone(), grades[3][2].clone(), grades[3][3].clone()]]}
    }

    impl<L1, L2> GradeSet<L1, L2> where L1: sec_lat::MoreSecretThan<L1> + secret_structs::ternary_lattice::Label, L2: int_lat::LowerIntegrityThan<L2> + secret_structs::ternary_lattice::Label
    {
        /*pub fn write_grade(&mut self, new_grade: i32, /*project_number: usize, */student_id: usize) {
            //let secret = &(self.grades[student_id][project_number]);
            //let all_grades = &mut self.grades;
            //let student_grades = &mut all_grades[student_id];
            //let secret = &mut student_grades[project_number];
            let student_grades = &mut self.grades;
            let secret = &mut student_grades[student_id];
            secret_structs::untrusted_secure_block_no_return_dynamic_all!(L1, L2, secret.get_dyn_sec_label_ref(), secret.get_dyn_int_label_ref(), {
                let unwrapped_secret: &mut i32 = unwrap_mut_ref(secret);
                *unwrapped_secret = new_grade;
            });
        }*/

        pub fn add_to_grade(&mut self, added: i32, /*project_number: usize, */student_id: usize) {
            //let secret = &(self.grades[student_id][project_number]);
            //let all_grades = &mut self.grades;
            //let student_grades = &mut all_grades[student_id];
            //let secret = &mut student_grades[project_number];
            let student_grades = &mut self.grades;
            let secret = &mut student_grades[student_id].0;
            secret_structs::untrusted_secure_block_no_return_dynamic_all!(L1, L2, secret.get_dyn_sec_label_ref(), secret.get_dyn_int_label_ref(), {
                let unwrapped_secret: &mut i32 = unwrap_mut_ref(secret);
                *unwrapped_secret += added;
            });
            //*unsafe { secret.unwrap_mut_unsafe_dynamic_all() } += added;
        }

        /*pub fn add_to_grade(&mut self, added: i32, project_number: usize, student_id: usize) {
            //let secret = &(self.grades[student_id][project_number]);
            let all_grades = &mut self.grades;
            let student_grades = &mut all_grades[student_id];
            let secret_non_mut = &student_grades[project_number];

            let secret_label = secret_non_mut.get_dyn_sec_label_ref();
            let integrity_label = secret_non_mut.get_dyn_int_label_ref();
            let secret = /*&student_grades[project_number];*/ &mut student_grades[project_number];
            //let secret_label = secret.get_dyn_sec_label_ref();
            //let integrity_label = secret.get_dyn_int_label_ref();
            secret_structs::untrusted_secure_block_no_return_dynamic_all!(L1, L2, secret_label, integrity_label, /*(&student_grades[project_number]).get_dyn_sec_label(), (&student_grades[project_number]).get_dyn_int_label(),*/ {
                /*let unwrapped_secret = unwrap_ref(secret);*/unwrap_mut_ref(secret);
                //*unwrapped_secret += added;
            });
        }*/*/

       /*pub fn add_to_grade(
            &mut self,
            added: i32,
            project_number: usize,
            student_id: usize,
        ) {
            let all_grades = &mut self.grades;
            let student_grades = &mut all_grades[student_id];
            let secret = &mut student_grades[project_number];
            {
                let block_secrecy: &DynamicSecretLabel = secret.get_dyn_sec_label_ref();
                let block_secrecy: &DynamicSecretLabel = unsafe { std::mem::transmute(block_secrecy) };
                //let block_secrecy = unsafe { std::ptr::read(block_secrecy) }; //SEE HERE
                let block_integrity: &DynamicIntegrityLabel = secret.get_dyn_int_label_ref();
                let block_integrity: &DynamicIntegrityLabel = unsafe { std::mem::transmute(block_integrity) };
                //let block_integrity = unsafe { std::ptr::read(block_integrity) }; //SEE HERE. NEED TO NOT DROP. OR DON'T CONVERT TO VALUE. SEE HOW TO CONSUME VAL WIHOUT DROP
                if true {
                    secret_structs::secret::call_info_flow_closure_no_return::<
                        L1,
                        L2,
                        _,
                    >(
                        (|| -> _ {
                            let result = std::panic::catch_unwind(
                                    ::std::panic::AssertUnwindSafe(|| {
                                        {
                                            {
                                                let unwrapped_secret: &mut i32 = {
                                                    let result = secret;
                                                    /*let b = result.get_dyn_sec_label_ref();
                                                    let c = b.compare_equal(&block_secrecy);*/
                                                    if !((result)
                                                        .get_dyn_sec_label_ref()
                                                        .compare_equal(&block_secrecy))
                                                    {
                                                        {
                                                            panic!(
                                                                "Principal tried to mutably unwrap data with a different secrecy than the block's.",
                                                            )
                                                        };
                                                    }
                                                    if !((&result)
                                                        .get_dyn_int_label_ref()
                                                        .compare_equal(&block_integrity))
                                                    {
                                                        {
                                                            panic!(
                                                                "Principal tried to mutably unwrap data with a different integrity than the block's.",
                                                            )
                                                        };
                                                    }
                                                    unsafe {
                                                        secret_structs::secret::SecureValue::unwrap_mut_unsafe_dynamic_all::<
                                                            L1,
                                                            L2,
                                                        >(result)
                                                    }
                                                };
                                                *unwrapped_secret += added;
                                            }
                                        }
                                    }),
                                )
                                .unwrap_or_default();
                            result
                        }),
                    )
                } else {
                    secret_structs::secret::call_info_flow_closure_no_return::<
                        L1,
                        L2,
                        _,
                    >(
                        (|| -> _ {
                            {
                                {
                                    let unwrapped_secret: &mut i32 = {
                                        unsafe {
                                            secret_structs::secret::SecureValue::unwrap_mut_unsafe_dynamic_all::<
                                                L1,
                                                L2,
                                            >({
                                                let tmp = &(secret);
                                                unsafe { ::secret_structs::secret::check_ISEF_unsafe(tmp) }
                                            })
                                        }
                                    };
                                    ::secret_structs::secret::SafeAddAssign::safe_add_assign(
                                        &mut *({
                                            let tmp = &(unwrapped_secret);
                                            unsafe { ::secret_structs::secret::check_ISEF_unsafe(tmp) }
                                        }),
                                        added,
                                    );
                                }
                            }
                        }),
                    )
                }
            };
        }
 */

        /*pub fn add_to_grade(
            &mut self,
            added: i32,
            project_number: usize,
            student_id: usize,
        ) {
            let all_grades = &mut self.grades;
            let student_grades = &mut all_grades[student_id];
            {
                let block_secrecy = (&student_grades[project_number])
                    .get_dyn_sec_label();
                let block_integrity = (&student_grades[project_number])
                    .get_dyn_int_label();
                if true {
                    secret_structs::secret::call_info_flow_closure_no_return::<
                        L1,
                        L2,
                        _,
                    >(
                        (|| -> _ {
                            let result = std::panic::catch_unwind(
                                    ::std::panic::AssertUnwindSafe(|| {
                                        {
                                            {
                                                let unwrapped_secret: &mut i32 = {
                                                    let result = &mut (student_grades[project_number]);
                                                    if !((&result)
                                                        .get_dyn_sec_label_ref()
                                                        .compare_equal(&block_secrecy))
                                                    {
                                                        {
                                                            panic!(
                                                                "Principal tried to mutably unwrap data with a different secrecy than the block's.",
                                                            )
                                                        };
                                                    }
                                                    if !((&result)
                                                        .get_dyn_int_label_ref()
                                                        .compare_equal(&block_integrity))
                                                    {
                                                        {
                                                            panic!(
                                                                "Principal tried to mutably unwrap data with a different integrity than the block's.",
                                                            )
                                                        };
                                                    }
                                                    unsafe {
                                                        secret_structs::secret::SecureValue::unwrap_mut_unsafe_dynamic_all::<
                                                            L1,
                                                            L2,
                                                        >(result)
                                                    }
                                                };
                                                *unwrapped_secret += added;
                                            }
                                        }
                                    }),
                                )
                                .unwrap_or_default();
                            result
                        }),
                    )
                } else {
                    secret_structs::secret::call_info_flow_closure_no_return::<
                        L1,
                        L2,
                        _,
                    >(
                        (|| -> _ {
                            {
                                {
                                    let unwrapped_secret: &mut i32 = {
                                        unsafe {
                                            secret_structs::secret::SecureValue::unwrap_mut_unsafe_dynamic_all::<
                                                L1,
                                                L2,
                                            >({
                                                ::secret_structs::secret::check_ISEF_mut_ref(
                                                    (&mut ::secret_structs::secret::check_safe_index_expr(
                                                        student_grades,
                                                    )[::secret_structs::secret::check_safe_index(
                                                        project_number,
                                                    )]),
                                                )
                                            })
                                        }
                                    };
                                    ::secret_structs::secret::SafeAddAssign::safe_add_assign(
                                        &mut *({
                                            let tmp = &(unwrapped_secret);
                                            unsafe { ::secret_structs::secret::check_ISEF_unsafe(tmp) }
                                        }),
                                        added,
                                    );
                                }
                            }
                        }),
                    )
                }
            };
        }
*/



        pub fn benchmark_average(&self, secret_label: &DynLabel<Sec>, integrity_label: &DynLabel<Int>) -> i32 {
            //let all_grades = &self.grades;
            let student_grades = &self.grades;
            let mut average = secret_structs::untrusted_secure_block_dynamic_all!(L1, L2, secret_label, integrity_label, {
                wrap(0)
            });
            //unsafe { secret_label.print_self(); }
            //unsafe { integrity_label.print_self(); }
            let count = self.grades.len();
            for i in 0..self.grades.len() {
                //let student_grades = &all_grades[i];
                //let secret = &student_grades[0];
                let secret = &student_grades[i].0;
                secret_structs::untrusted_secure_block_no_return_dynamic_all!(L1, L2, secret_label, integrity_label, {
                    let unwrapped_secret: &i32 = unwrap_ref(&secret);
                    let unwrapped_average: &mut i32 = unwrap_mut_ref(&mut average);
                    *unwrapped_average += *unwrapped_secret;
                });
            }
            secret_structs::untrusted_secure_block_no_return_dynamic_all!(L1, L2, secret_label, integrity_label, {
                let unwrapped_average: &mut i32 = unwrap_mut_ref(&mut average);
                *unwrapped_average /= count as i32;
            });
            secret_structs::trusted_secure_block_dynamic_all!(L1, L2, secret_label, integrity_label, {
                unwrap(average)
            })
        }

        /*pub fn read_grade(&self, project_number: usize, student_id: usize) {
            //let all_grades = &self.grades;
            //let student_grades = &all_grades[student_id];
            //let secret = &student_grades[project_number];
            let student_grades = &self.grades;
            let secret = &student_grades[student_id];
            let grade: SecureValue<i32, L1, L2, DynLabel<Sec>, DynLabel<Int>> = secret_structs::untrusted_secure_block_dynamic_all!(L1, L2, secret.get_dyn_sec_label_ref(), secret.get_dyn_int_label_ref(), {
                //let secret = &self.grades[student_id][project_number];
                let unwrapped_secret: &i32 = unwrap_ref(&secret);
                // modify unwrapped_secret in some way:
                let new_secret = core::primitive::i32::clone(unwrapped_secret);
                wrap(new_secret)
            });
            let printable_grade = secret_structs::trusted_secure_block_dynamic_all!(L1, L2, grade.get_dyn_sec_label_ref(), grade.get_dyn_int_label_ref(),  {
                unwrap(grade)
            });
            //let printable_grade = grade.declassify_ref();
        }*/

        /*pub fn compute_student_average(&self, student_id: usize) {
            let all_grades = &self.grades;
            let student_grades = &all_grades[student_id];
            let mut average = 0;
            for i in 0..4 {
                let secret = &student_grades[i];
                let grade = secret_structs::untrusted_secure_block_dynamic_all!(L1, L2, secret.get_dyn_sec_label_ref(), secret.get_dyn_int_label_ref(),  {
                    //let secret = &self.grades[student_id][project_number];
                    let unwrapped_secret: &i32 = unwrap_ref(&secret);
                    // modify unwrapped_secret in some way:
                    let new_secret = core::primitive::i32::clone(unwrapped_secret);
                    wrap(new_secret)
                });
                let printable_grade = secret_structs::trusted_secure_block_dynamic_all!(L1, L2, grade.get_dyn_sec_label_ref(), grade.get_dyn_int_label_ref(),  {
                    unwrap(grade)
                });
                average += printable_grade;
            }
            average /= 4;
        }

        pub fn compute_project_average(&self, project_number: usize) {
            let all_grades = &self.grades;
            let mut average = 0;
            for i in 0..4 {
                let student_grades = &all_grades[i];
                let secret = &student_grades[project_number];
                let printable_grade = secret_structs::trusted_secure_block_dynamic_all!(L1, L2, secret.get_dyn_sec_label_ref(), secret.get_dyn_int_label_ref(), {
                    let grade = secret_structs::untrusted_secure_block_dynamic_all!(L1, L2, secret.get_dyn_sec_label_ref(), secret.get_dyn_int_label_ref(), {
                        //let secret = &self.grades[student_id][project_number];
                        let unwrapped_secret: &i32 = unwrap_ref(&secret);
                        // modify unwrapped_secret in some way:
                        let new_secret = core::primitive::i32::clone(unwrapped_secret);
                        wrap(new_secret)
                    });
                    unwrap(grade)
                });
                average += printable_grade;
            }
            average /= 4;
        }*/
    }
}