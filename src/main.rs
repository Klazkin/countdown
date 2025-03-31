use chrono::{DateTime, Datelike, Month, Months, TimeZone, Timelike, Utc};
use gloo::timers::callback::Interval;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <Stats/>
            <Calendar/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CalendarYearProp {
    year: i32,
    months: Vec<CalendarMonthProp>,
}

#[function_component(CalendarYear)]
fn calendar_year(CalendarYearProp { year, months }: &CalendarYearProp) -> Html {
    let (year_class, year_sep_class) = if Utc::now().year() > *year {
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
                    <CalendarMonth year={prop.year} month={prop.month} completion={prop.completion.clone()} duration={prop.duration.clone()}/>
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

    let weekdays_names = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]; // todo use monospace font
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

    let (elapsed, remaining) = match *completion {
        MonthCompletion::Completed => (days_in_month, 0),
        MonthCompletion::NotStarted => (0, days_in_month),
        MonthCompletion::Partial(d) => (d, days_in_month - d),
    };

    let div_class = match completion {
        MonthCompletion::Completed => "month-completed",
        MonthCompletion::NotStarted => "month",
        MonthCompletion::Partial(_) => "month-current",
    };

    let day_color = day_color();
    let month_style = if matches!(completion, MonthCompletion::Partial(_)) {
        format!("color:{}; border_color:{}", &day_color, &day_color)
    } else {
        String::new()
    };

    let day_completion_tag = format!(
        "width:{}px; background-color:{}",
        (Utc::now().hour() as f32) / 24.0 * 16.0,
        day_color
    );

    let day_style = if matches!(completion, MonthCompletion::Partial(_)) {
        format!("background-color:{}", &day_color)
    } else {
        String::new()
    };

    html! {
        <div class={ div_class } style={month_style}>

            <div style="padding-left: 0.5em">{ month_name }</div>

            <div class="day-container">
                { weekdays_names.iter().map(|s| html! {
                    <div>{*s}</div>
                }).collect::<Html>() }

                { (0..(offset % 7)).map(|_| html!{
                    <div class="day-filler"></div>
                }).collect::<Html>() }

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
fn calendar() -> Html {
    let now = Utc::now();
    let start = Utc.with_ymd_and_hms(2024, 7, 16, 12, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 6, 13, 12, 0, 0).unwrap();

    let mut calendar = HashMap::<i32, CalendarYearProp>::new();
    let mut date = Utc
        .with_ymd_and_hms(start.year(), start.month(), 1, 0, 0, 0)
        .unwrap();

    while date.with_day(1) <= end.with_day(1) {
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

        let completion = if now > date {
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

    calendar
        .values()
        .map(|cld| {
            html! {
                <CalendarYear year={cld.year} months={cld.months.clone()}/>
            }
        })
        .collect()
}

struct Stats {
    now: DateTime<Utc>,
    handle: Interval,
}

impl Component for Stats {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let handle = {
            let link = ctx.link().clone();
            Interval::new(16, move || link.send_message(()))
        };

        Self {
            now: Utc::now(),
            handle,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.now = Utc::now();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let month = Month::try_from(self.now.month() as u8).unwrap().name();
        let year = self.now.year();
        let day_suffix = match self.now.day() % 10 {
            0 | 4 | 5 | 6 | 7 | 8 | 9 => "th",
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => panic!(),
        };
        let day = self.now.day();

        let start = Utc.with_ymd_and_hms(2024, 7, 16, 12, 0, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2025, 6, 13, 12, 0, 0).unwrap();

        let total_delta = end - start;
        let current_delta = end - Utc::now();
        let factor = 1.0
            - (current_delta.num_milliseconds() as f64) / (total_delta.num_milliseconds() as f64);

        html! {
            <div class="stats">
                <h3>
                    { format!("{day}{day_suffix} of {month}, {year}") }
                </h3>

                 <div>
                    { format!("Day {} of {} ({} left)", total_delta.num_days() - current_delta.num_days(), total_delta.num_days(), current_delta.num_days()) }
                </div>

                <div>
                    {format!("Seconds left: {:.3}s", current_delta.num_milliseconds() as f64 / 1000.0)}
                </div>

                <div class="progress-bar">
                    <div class="progress-bar-text"> { format!("{:.20}%", factor * 100.0) } </div>
                    <div class="progress-bar-fill" style={format!("width:{}%", factor * 100.0)}></div>
                 </div>
            </div>
        }
    }
}

fn day_color() -> String {
    let date = Utc::now();
    let seed = (date.year() as u64) * 10000 + (date.month() as u64) * 100 + (date.day() as u64);
    let mut rng = StdRng::seed_from_u64(seed);

    format!(
        "#{:02X}{:02X}{:02X}",
        rng.random::<u8>(),
        rng.random::<u8>(),
        rng.random::<u8>()
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
