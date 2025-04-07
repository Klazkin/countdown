use crate::{end, now, start};
use chrono::{DateTime, Datelike, Month, Utc};
use gloo::timers::callback::Interval;
use yew::{Component, Context, Html, html};

pub struct Stats {
    now: DateTime<Utc>,
    _handle: Interval,
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
            now: now(),
            _handle: handle,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.now = now();
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
            _ => unreachable!(),
        };
        let day = self.now.day();
        let total_delta = end() - start();
        let elapsed_delta = self.now - start();
        let remaining_delta = total_delta - elapsed_delta;
        let factor = (elapsed_delta.num_milliseconds() as f64)
            / (total_delta.num_milliseconds() as f64)
            * 100.0;

        let time_left_formatted = format!(
            "{}:{:0>2}:{:0>2}.{:0>3}",
            remaining_delta.num_hours(),
            remaining_delta.num_minutes() % 60,
            remaining_delta.num_seconds() % 60,
            remaining_delta.num_milliseconds() % 1000,
        );

        html! {
            <div class="stats">
                <h3>{ format!("{day}{day_suffix} of {month}, {year}") }</h3>

                 <div>
                    { format!("Day {} of {} ({} left)",
                        elapsed_delta.num_days() + 1, // account for ongoing day
                        total_delta.num_days(),
                        remaining_delta.num_days() - 1,
                    )}
                </div>

                <div>
                    {format!("{time_left_formatted} (HH:MM:SS.mm)")}
                </div>

                <div class="progress-bar">
                    <div class="progress-bar-text"> { format!("{factor:.20}%") } </div>
                    <div class="progress-bar-fill" style={format!("width:{factor}%")}></div>
                 </div>
            </div>
        }
    }
}
