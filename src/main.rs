mod calendar;
mod stats;

use crate::calendar::Calendar;
use crate::stats::Stats;

use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
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

fn now() -> DateTime<Utc> {
    Utc::now()
}

fn start() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2024, 7, 16, 12, 0, 0).unwrap()
}

fn end() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2025, 6, 13, 12, 0, 0).unwrap()
}

fn day_color() -> String {
    let date = now();
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
