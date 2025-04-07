use crate::{day_color, end, now, start};
use chrono::{Datelike, Month, Months, TimeZone, Timelike, Utc};
use std::collections::HashMap;
use yew::{Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
struct CalendarYearProp {
    year: i32,
    months: Vec<CalendarMonthProp>,
}

#[function_component(CalendarYear)]
fn calendar_year(CalendarYearProp { year, months }: &CalendarYearProp) -> Html {
    let (year_class, year_sep_class) = if now().year() > *year {
        ("year-completed", "year-sep-completed")
    } else {
        ("year", "year-sep")
    };

    html! {
        <div class={year_class}>
            { year.to_string() }
            <div class={year_sep_class}></div>
            <div class="month-grid-container">
                {
                    months.iter().map(|prop| html! {
                    <CalendarMonth
                        year={prop.year}
                        month={prop.month}
                        completion={prop.completion.clone()}
                        duration={prop.duration.clone()}/>
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

#[derive(Debug, PartialEq, Clone)]
enum MonthCompletion {
    Completed,
    NotStarted,
    Partial(u8),
}

#[derive(Debug, PartialEq, Clone)]
enum MonthDuration {
    Full,
    StartOffset(u8),
    EndOffset(u8),
}

#[derive(Debug, Properties, PartialEq, Clone)]
struct CalendarMonthProp {
    year: i32,
    month: u32,
    completion: MonthCompletion,
    duration: MonthDuration,
}

#[function_component(CalendarMonth)]
fn calendar_months(
    CalendarMonthProp {
        year,
        month,
        completion,
        duration,
    }: &CalendarMonthProp,
) -> Html {
    let (year, month) = (*year, *month);
    let weekdays_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let mut offset = Utc
        .with_ymd_and_hms(year, month, 1, 0, 0, 0)
        .unwrap()
        .weekday()
        .num_days_from_monday() as u8;
    let month_as_enum = Month::try_from(month as u8).unwrap();
    let month_name = month_as_enum.name();
    let mut days_in_month = month_as_enum.num_days(year).unwrap();

    match *duration {
        MonthDuration::Full => {}
        MonthDuration::StartOffset(d) => {
            offset = offset + d;
            days_in_month = days_in_month - d;
        }
        MonthDuration::EndOffset(d) => {
            days_in_month = d;
        }
    };

    let (elapsed, remaining, div_class) = match *completion {
        MonthCompletion::Completed => (days_in_month, 0, "month-completed"),
        MonthCompletion::NotStarted => (0, days_in_month, "month"),
        MonthCompletion::Partial(d) => (d, days_in_month - d, "month-current"),
    };

    let day_color = day_color();

    let (month_style, day_style) = if matches!(completion, MonthCompletion::Partial(_)) {
        (
            format!("color:{}; border_color:{}", &day_color, &day_color),
            format!("background-color:{}", &day_color),
        )
    } else {
        (String::new(), String::new())
    };

    let day_completion_tag = format!(
        "width:{}px; background-color:{}",
        (now().hour() as f32) / 24.0 * 16.0,
        day_color
    );

    html! {
        <div class={ div_class } style={month_style}>
            <div class="month-name">{ month_name }</div>
            <div class="day-container">
                { weekdays_names.iter().map(|s| html! {<div>{*s}</div>}).collect::<Html>() }

                { (0..(offset % 7)).map(|_| html!{<div></div>}).collect::<Html>() }

                { (0..elapsed).map(|_| html! {
                    <div class="day-completed" style={day_style.clone()}></div>
                }).collect::<Html>() }

                { (0..remaining).map(|d| html! {
                    <div class="day">
                    if d == 0 && matches!(completion, MonthCompletion::Partial(_)) {
                        <div class="current-day" style={day_completion_tag.clone()}></div>
                    }
                    </div>
                }).collect::<Html>() }
            </div>
        </div>
    }
}

#[function_component(Calendar)]
pub fn calendar() -> Html {
    let (start, end, now) = (start(), end(), now());
    let mut calendar = HashMap::<i32, CalendarYearProp>::new();

    let mut date = Utc
        .with_ymd_and_hms(start.year(), start.month(), 1, 0, 0, 0)
        .unwrap();

    while date.with_day(1) <= end.with_day(1) {
        // iterates through every month
        let cal_year = calendar.entry(date.year()).or_insert(CalendarYearProp {
            year: date.year().to_owned(),
            months: Default::default(),
        });

        let duration = if (date.month(), date.year()) == (start.month(), start.year()) {
            MonthDuration::StartOffset(start.day() as u8)
        } else if (date.month(), date.year()) == (end.month(), end.year()) {
            MonthDuration::EndOffset(end.day() as u8)
        } else {
            MonthDuration::Full
        };

        let completion = if now >= date {
            if now.month() == date.month() {
                MonthCompletion::Partial(now.day() as u8 - 1)
            } else {
                MonthCompletion::Completed
            }
        } else {
            MonthCompletion::NotStarted
        };

        cal_year.months.push(CalendarMonthProp {
            year: date.year(),
            month: date.month(),
            completion,
            duration,
        });

        date = date.checked_add_months(Months::new(1)).unwrap();
    }

    let mut years = calendar.values().collect::<Vec<&CalendarYearProp>>();

    years.sort_by(|a, b| a.year.cmp(&b.year));

    years
        .into_iter()
        .map(|cld| {
            html! {
                <CalendarYear year={cld.year} months={cld.months.clone()}/>
            }
        })
        .collect()
}
