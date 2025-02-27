use chrono::{prelude::*, Duration};
use itertools::Itertools;

use crate::events::Event;

use super::availability::Availability;

use secret_macros::side_effect_free_attr;
use secret_macros::side_effect_free_attr_trait;
use secret_macros::InvisibleSideEffectFreeDerive;
use secret_structs::*;
use secret_structs::secret::*;
use secret_structs::integrity_lattice as int_lat;
use secret_structs::ternary_lattice as sec_lat;
use indicatif::ProgressBar;

#[derive(InvisibleSideEffectFreeDerive)]
pub struct AvailabilityFinder {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub min: NaiveTime,
    pub max: NaiveTime,
    pub duration: Duration,
    pub include_weekends: bool,
}

#[side_effect_free_attr]
fn is_weekend(weekday: Weekday) -> bool {
    unchecked_operation(weekday == Weekday::Sat || weekday == Weekday::Sun)
}

#[allow(clippy::type_complexity)]
impl AvailabilityFinder {
    pub fn get_availability(
        &self,
        mut events: SecureValue<Vec<Event>, sec_lat::Label_Empty, int_lat::Label_All, DynLabel<Sec>, ()>,
        mut pb: &ProgressBar,
    ) -> anyhow::Result<SecureValue<Vec<(Date<Local>, Vec<Availability<Local>>)>, sec_lat::Label_Empty, int_lat::Label_All, DynLabel<Sec>, ()>> {
        pb.set_message("Start...");
        let mut avail: SecureValue<Vec<(Date<Local>, Vec<Availability<Local>>)>, sec_lat::Label_Empty, int_lat::Label_All, DynLabel<Sec>, ()> = untrusted_secure_block_dynamic_secrecy!(sec_lat::Label_Empty, int_lat::Label_All, events.get_dyn_sec_label_ref(), {
            wrap(std::vec::Vec::new())
        }); //vec![];

        //events.sort_by_key(|e| e.start);

        untrusted_secure_block_no_return_dynamic_secrecy!(sec_lat::Label_Empty, int_lat::Label_All, events.get_dyn_sec_label_ref(), {
            unwrap_mut_ref(&mut events).sort_by_key(|e: &Event| e.start);
        });
        pb.set_message("Passed sort_by_key...");
        //events.sort_by_key(|e| e.start);

        pub struct GroupBy2 {
            groups: std::vec::IntoIter<Event>,
            item: Option<Event>,
        }

        unsafe impl InvisibleSideEffectFree for GroupBy2 {}

        impl Iterator for GroupBy2 {
            type Item = (Date<Local>, Vec<Event>);
            fn next(&mut self) -> Option<(Date<Local>, Vec<Event>)> {
                if let Some(event) = /*&self.item*/ std::mem::replace(&mut self.item, None) {
                    let mut group = Vec::<Event>::new();
                    let key = (&event).start.date();
                    group.push(event);
                    let mut item = self.groups.next();
                    while let Some(event2) = item {
                        if (&event2).start.date() == key {
                            group.push(event2);
                            item = self.groups.next();
                        } else {
                            self.item = Some(event2);
                            return Some((key, group));
                        }
                    } 
                    self.item = None;
                    return Some((key, group));
                } else {
                    return None
                }
            }
        }

        fn new_group_by_2(mut groups: std::vec::IntoIter<Event>) -> GroupBy2 {
            let mut g = groups;
            let i = g.next();
            GroupBy2 {
                groups: g,
                item: i,
            }
        }

        let mut iter = untrusted_secure_block_dynamic_secrecy!(sec_lat::Label_Empty, int_lat::Label_All, events.get_dyn_sec_label_ref(), {
            let mut days = unwrap(events).into_iter();//.group_by(|e| (e.start.date()));
            let new_group_by_2 = explicit_allowlisted(new_group_by_2(days));
            wrap(new_group_by_2)
        });
        pb.set_message("Passed creation of iter...");
        // Start at start day and min time
       
        
        let one_day = Duration::days(1);
        untrusted_secure_block_no_return_dynamic_secrecy!(sec_lat::Label_Empty, int_lat::Label_All, iter.get_dyn_sec_label_ref(), {
            let mut curr = self.
                start
                .date()
                .and_hms(self.min.hour(), self.min.minute(), 0);
            curr = explicit_allowlisted(DateTime::max(curr, self.start));
            curr = curr.ceil();

            while unchecked_operation(curr < self.end) {
                //let day = iter.next();
                let day = explicit_allowlisted(GroupBy2::next(unwrap_mut_ref(&mut iter)));
                let (date_opt, events_opt, is_some) = match day {
                    Some((date, events)) => (std::option::Option::Some(date), std::option::Option::Some(events), true),
                    None => (None, None, false),
                };

                // Have another day of events to process
                if /*let Some((date, events)) = day*/ is_some {

                    let date = date_opt.unwrap();
                    let events = events_opt.unwrap();
                    // Add days that are entirely free
                    //
                    // If curr.date < date and curr.time < max, then we advance to the start of the next day
                    

                    while {let compared_date = DateTime::<Local>::date(&curr); unchecked_operation(compared_date < date)} {
                        if {let c_time = curr.time(); unchecked_operation(c_time < self.max)} {
                            let end = curr.date().and_hms(self.max.hour(), self.max.minute(), 0);
                            if self.include_weekends || !is_weekend(curr.weekday()) {
                                unwrap_mut_ref(&mut avail).push((
                                    curr.date(),
                                    std::vec::from_elem(Availability {
                                        start: curr.date().and_hms(
                                            self.min.hour(),
                                            self.max.minute(),
                                            0,
                                        ),
                                        end,
                                    }, 1),
                                ));
                            }
                        }
                        curr = (unchecked_operation(curr + one_day)).date().and_hms(
                            self.min.hour(),
                            self.min.minute(),
                            0,
                        );
                    }

                    if !self.include_weekends && is_weekend(Date::<Local>::weekday(&date)) {
                        if {let compared_date = DateTime::<Local>::date(&curr); unchecked_operation(compared_date == date)} {
                            curr = (unchecked_operation(curr + one_day)).date().and_hms(
                                self.min.hour(),
                                self.min.minute(),
                                0,
                            );
                        }
                        continue;
                    }

                    // events is guaranteed to be non-empty because of the GroupBy

                    // Check for availabilities within the day

                    let mut day_avail = std::vec::Vec::new();
                    let mut curr_time = self.min;

                    for event in events {
                        let start = event.start;
                        let end = event.end;
                        let start_time = start.time();
                        if unchecked_operation(curr_time < start_time) {
                            let avail_start = start
                                .date()
                                .and_hms(curr_time.hour(), curr_time.minute(), 0)
                                .ceil();

                            let param = curr.date().and_hms(self.max.hour(), self.max.minute(), 0);
                            let avail_end = explicit_allowlisted(DateTime::min(start, param)).floor();

                            let e_t = avail_end.time();
                            let s_t = avail_start.time();
                            if unchecked_operation(e_t - s_t >= self.duration
                                && s_t < self.max)
                            {
                                day_avail.push(Availability {
                                    start: avail_start,
                                    end: avail_end,
                                });
                            }

                        }
                        let e_time = end.time();
                        curr_time = explicit_allowlisted(NaiveTime::max(e_time, curr_time));
                    }

                    if unchecked_operation(curr_time < self.max) {
                        let avail_start = curr
                            .date()
                            .and_hms(curr_time.hour(), curr_time.minute(), 0)
                            .ceil();
                        let avail_end = curr.date().and_hms(self.max.hour(), self.max.minute(), 0);

                        if unchecked_operation(avail_end - avail_start >= self.duration) {
                            day_avail.push(Availability {
                                start: avail_start,
                                end: avail_end,
                            });
                        }
                    }

                    // Still have time left over today.
                    // TODO: combine with logic in the else below
                    unwrap_mut_ref(&mut avail).push((curr.date(), day_avail));

                    // 12AM next day
                    curr = (unchecked_operation(curr + one_day)).date().and_hms(
                        self.min.hour(),
                        self.min.minute(),
                        0,
                    );
                } else {
                    // Add days that are entirely free
                    // Either before end date or on the end date but before the max time
                    while {let c_date = curr.date(); let e_date = self.end.date(); unchecked_operation(c_date < e_date
                        || (c_date == e_date && curr < self.end))}
                    {
                        if !is_weekend(curr.weekday()) || self.include_weekends {
                            let start = curr.ceil();

                            // Whole day
                            let s_time = start.time();
                            let end = unchecked_operation(curr + (self.max - s_time));

                            if unchecked_operation(s_time <= self.max && end - start >= self.duration) {
                                unwrap_mut_ref(&mut avail).push((curr.date(), std::vec::from_elem(Availability { start, end }, 1)));
                            }
                        }

                        // min next day
                        curr = (unchecked_operation(curr + one_day)).date().and_hms(
                            self.min.hour(),
                            self.min.minute(),
                            0,
                        );
                    }
                }
            }
        });
        pb.set_message("End...");
        Ok(avail)
    }
}

pub trait Round: Sized + InvisibleSideEffectFree {
    #[side_effect_free_attr_trait(method)]
    fn ceil(&self) -> Self;
    #[side_effect_free_attr_trait(method)]
    fn floor(&self) -> Self;
}

impl<T: TimeZone> Round for DateTime<T> {
    #[side_effect_free_attr(method)]
    fn ceil(&self) -> Self {
        let time = self.date().and_hms(self.hour(), self.minute(), 0);
        let minute = self.minute();

        let round_to_minute = 30;

        if minute % round_to_minute == 0 {
            return time;
        }

        let new_minute = (minute / round_to_minute + 1) * round_to_minute;

        let sub = new_minute - minute;

        // Carapace allowlisted
        let duration = explicit_allowlisted(Duration::minutes(unchecked_operation(sub.into())));
        unchecked_operation(time + duration)
    }

    #[side_effect_free_attr(method)]
    fn floor(&self) -> Self {
        let time = self.date().and_hms(self.hour(), self.minute(), 0);

        let round_to_minute: i64 = 30;

        let pre_minute = self.minute();
        let minute: i64 = unchecked_operation(pre_minute.into());

        if minute % round_to_minute == 0 {
            return time;
        }

        let new_minute = (minute / round_to_minute) * round_to_minute;

        let delta: i64 = new_minute - minute;

        // Carapace allowlisted
        let duration = explicit_allowlisted(Duration::minutes(delta));
        unchecked_operation(time + duration)
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::*;

    fn create_local_datetime(dt_str: &str) -> DateTime {
        let datetime_fmt = "%m-%d-%Y %H:%M";
        let ndt = NaiveDateTime::parse_from_str(dt_str, datetime_fmt).unwrap();
        Local.from_local_datetime(&ndt).unwrap()
    }

    #[test]
    fn test_round_datetime_up() {
        let dt = create_local_datetime("10-05-2022 00:00");
        assert_eq!(dt, dt.ceil());

        let dt = create_local_datetime("10-05-2022 00:02");
        assert_eq!(create_local_datetime("10-05-2022 00:30"), dt.ceil());

        let dt = create_local_datetime("10-05-2022 00:42");
        assert_eq!(create_local_datetime("10-05-2022 01:00"), dt.ceil());

        // Next day
        let dt = create_local_datetime("10-05-2022 23:42");
        assert_eq!(create_local_datetime("10-06-2022 00:00"), dt.ceil());

        // Should disregard seconds
        let dt = create_local_datetime("10-05-2022 00:02") + Duration::seconds(30);
        assert_eq!(create_local_datetime("10-05-2022 00:30"), dt.ceil());
    }

    #[test]
    fn test_round_datetime_down() {
        let dt = create_local_datetime("10-05-2022 00:00");
        assert_eq!(dt, dt.floor());

        let dt2 = create_local_datetime("10-05-2022 00:02");
        assert_eq!(dt, dt2.floor());

        let dt3 = create_local_datetime("10-05-2022 00:42");
        assert_eq!(create_local_datetime("10-05-2022 00:30"), dt3.floor());

        // Should disregard seconds
        let dt4 = create_local_datetime("10-05-2022 00:02") + Duration::seconds(30);
        assert_eq!(dt, dt4.floor());
    }

    fn create_event(start: &str, end: &str) -> Event {
        let event_id = "id";
        let event_name = "name";
        Event {
            id: event_id.to_string(),
            name: Some(event_name.to_string()),
            // 12 PM
            start: create_local_datetime(start),
            // 2 PM
            end: create_local_datetime(end),
        }
    }

    #[test]
    fn test_get_availability() {
        let events = vec![
            // 12pm - 2pm
            create_event("10-05-2022 12:00", "10-05-2022 14:00"),
            // 3:30pm - 4pm
            create_event("10-05-2022 15:30", "10-05-2022 16:00"),
            // 4pm - 6pm
            create_event("10-05-2022 16:00", "10-05-2022 18:00"),
            // 7pm - 9pm (outside min-max window)
            create_event("10-05-2022 19:00", "10-05-2022 21:00"),
            // Next day, 5:30am to 7am (outside min-max window)
            create_event("10-06-2022 05:30", "10-06-2022 07:00"),
            // Next day, 8:30am to 12pm
            create_event("10-06-2022 08:30", "10-06-2022 12:00"),
        ];

        let finder = AvailabilityFinder {
            start: create_local_datetime("10-05-2022 00:00"),
            end: create_local_datetime("10-07-2022 00:00"),
            min: NaiveTime::from_hms(9, 0, 0),
            max: NaiveTime::from_hms(17, 0, 0),
            duration: Duration::minutes(30),
            include_weekends: true,
        };
        let avails = finder.get_availability(events).unwrap();

        assert_eq!(avails.len(), 2);
        let mut day_avails = &avails.get(0).unwrap().1;
        assert_eq!(day_avails.len(), 2);

        assert_eq!(
            *day_avails.get(0).unwrap(),
            Availability {
                start: create_local_datetime("10-05-2022 09:00"),
                end: create_local_datetime("10-05-2022 12:00"),
            }
        );
        assert_eq!(
            *day_avails.get(1).unwrap(),
            Availability {
                start: create_local_datetime("10-05-2022 14:00"),
                end: create_local_datetime("10-05-2022 15:30"),
            }
        );

        day_avails = &avails.get(1).unwrap().1;
        assert_eq!(day_avails.len(), 1);
        assert_eq!(
            *day_avails.get(0).unwrap(),
            Availability {
                start: create_local_datetime("10-06-2022 12:00"),
                end: create_local_datetime("10-06-2022 17:00"),
            }
        );
    }

    #[test]
    fn test_get_availability_without_weekends() {
        let events = vec![
            // 12pm - 2pm, Friday
            create_event("11-18-2022 12:00", "11-18-2022 14:00"),
            // 3:30pm - 5pm, Friday
            create_event("11-18-2022 15:30", "11-18-2022 17:00"),
            // 3pm - 5pm, Saturday
            create_event("11-19-2022 15:00", "11-19-2022 17:00"),
            // Monday, 8:30am to 11am
            create_event("11-21-2022 08:30", "11-21-2022 11:00"),
            // Monday, 1pm to 2pm
            create_event("11-21-2022 13:00", "11-21-2022 14:00"),
        ];

        let finder = AvailabilityFinder {
            start: create_local_datetime("11-18-2022 00:00"),
            end: create_local_datetime("11-22-2022 00:00"),
            min: NaiveTime::from_hms(9, 0, 0),
            max: NaiveTime::from_hms(17, 0, 0),
            duration: Duration::minutes(30),
            include_weekends: false,
        };
        let avails = finder.get_availability(events).unwrap();

        assert_eq!(avails.len(), 2);
        let mut day_avails = &avails.get(0).unwrap().1;
        assert_eq!(day_avails.len(), 2);

        assert_eq!(
            *day_avails.get(0).unwrap(),
            Availability {
                start: create_local_datetime("11-18-2022 09:00"),
                end: create_local_datetime("11-18-2022 12:00"),
            }
        );
        assert_eq!(
            *day_avails.get(1).unwrap(),
            Availability {
                start: create_local_datetime("11-18-2022 14:00"),
                end: create_local_datetime("11-18-2022 15:30"),
            }
        );

        day_avails = &avails.get(1).unwrap().1;
        assert_eq!(day_avails.len(), 2);
        assert_eq!(
            *day_avails.get(0).unwrap(),
            Availability {
                start: create_local_datetime("11-21-2022 11:00"),
                end: create_local_datetime("11-21-2022 13:00"),
            }
        );
        assert_eq!(
            *day_avails.get(1).unwrap(),
            Availability {
                start: create_local_datetime("11-21-2022 14:00"),
                end: create_local_datetime("11-21-2022 17:00"),
            }
        );
    }

    #[test]
    fn test_get_availability_rounding() {
        let events = vec![
            // 11:55am - 12:35pm
            create_event("10-05-2022 11:55", "10-05-2022 12:35"),
            // 1:35pm - 2:10pm
            create_event("10-05-2022 13:35", "10-05-2022 14:10"),
            // 3:30pm - 4:05pm
            create_event("10-05-2022 15:30", "10-05-2022 16:05"),
        ];
        let finder = AvailabilityFinder {
            start: create_local_datetime("10-05-2022 00:00"),
            end: create_local_datetime("10-06-2022 00:00"),
            min: NaiveTime::from_hms(9, 0, 0),
            max: NaiveTime::from_hms(17, 0, 0),
            duration: Duration::minutes(30),
            include_weekends: true,
        };
        let avails = finder.get_availability(events).unwrap();

        assert_eq!(avails.len(), 1);
        let day_avails = &avails.get(0).unwrap().1;
        assert_eq!(day_avails.len(), 4);

        assert_eq!(
            *day_avails.get(0).unwrap(),
            Availability {
                start: create_local_datetime("10-05-2022 09:00"),
                end: create_local_datetime("10-05-2022 11:30"),
            }
        );
        assert_eq!(
            *day_avails.get(1).unwrap(),
            Availability {
                start: create_local_datetime("10-05-2022 13:00"),
                end: create_local_datetime("10-05-2022 13:30"),
            }
        );
        assert_eq!(
            *day_avails.get(2).unwrap(),
            Availability {
                start: create_local_datetime("10-05-2022 14:30"),
                end: create_local_datetime("10-05-2022 15:30"),
            }
        );
        assert_eq!(
            *day_avails.get(3).unwrap(),
            Availability {
                start: create_local_datetime("10-05-2022 16:30"),
                end: create_local_datetime("10-05-2022 17:00"),
            }
        );
    }

    #[test]
    fn test_get_availability_no_events() {
        let finder = AvailabilityFinder {
            start: create_local_datetime("10-05-2022 00:00"),
            end: create_local_datetime("10-07-2022 00:00"),
            min: NaiveTime::from_hms(9, 0, 0),
            max: NaiveTime::from_hms(17, 0, 0),
            duration: Duration::minutes(30),
            include_weekends: true,
        };
        let avails = finder.get_availability(vec![]).unwrap();

        assert_eq!(avails.len(), 2);
        let mut day_avails = &avails.get(0).unwrap().1;
        assert_eq!(day_avails.len(), 1);
        assert_eq!(
            *day_avails.get(0).unwrap(),
            Availability {
                start: create_local_datetime("10-05-2022 09:00"),
                end: create_local_datetime("10-05-2022 17:00"),
            }
        );

        day_avails = &avails.get(1).unwrap().1;
        assert_eq!(day_avails.len(), 1);
        assert_eq!(
            *day_avails.get(0).unwrap(),
            Availability {
                start: create_local_datetime("10-06-2022 09:00"),
                end: create_local_datetime("10-06-2022 17:00"),
            }
        );
    }

    #[test]
    fn test_get_availability_start_with_full_day() {
        let events = vec![
            // No events on start day

            // 12pm - 2pm
            create_event("10-06-2022 12:00", "10-06-2022 14:00"),
            // 3:30pm - 4pm
            create_event("10-06-2022 15:30", "10-06-2022 16:00"),
        ];
        let finder = AvailabilityFinder {
            start: create_local_datetime("10-05-2022 00:00"),
            end: create_local_datetime("10-07-2022 00:00"),
            min: NaiveTime::from_hms(9, 0, 0),
            max: NaiveTime::from_hms(17, 0, 0),
            duration: Duration::minutes(30),
            include_weekends: true,
        };
        let avails = finder.get_availability(events).unwrap();

        assert_eq!(avails.len(), 2);
        let mut day_avails = &avails.get(0).unwrap().1;
        assert_eq!(day_avails.len(), 1);
        assert_eq!(
            *day_avails.get(0).unwrap(),
            // Full day
            Availability {
                start: create_local_datetime("10-05-2022 09:00"),
                end: create_local_datetime("10-05-2022 17:00"),
            }
        );

        day_avails = &avails.get(1).unwrap().1;
        assert_eq!(day_avails.len(), 3);
        assert_eq!(
            *day_avails.get(0).unwrap(),
            Availability {
                start: create_local_datetime("10-06-2022 09:00"),
                end: create_local_datetime("10-06-2022 12:00"),
            }
        );
        assert_eq!(
            *day_avails.get(1).unwrap(),
            Availability {
                start: create_local_datetime("10-06-2022 14:00"),
                end: create_local_datetime("10-06-2022 15:30"),
            }
        );
        assert_eq!(
            *day_avails.get(2).unwrap(),
            Availability {
                start: create_local_datetime("10-06-2022 16:00"),
                end: create_local_datetime("10-06-2022 17:00"),
            }
        );
    }
}
