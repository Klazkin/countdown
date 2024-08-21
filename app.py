from dataclasses import dataclass

from flask import Flask, render_template
from datetime import datetime, timedelta

dt = datetime

start = dt(2024, 7, 16, 12, 30, 0)
now = dt.now()
end = dt(2025, 6, 18, 12)
day_total = (end.date() - start.date()).days

sbk_end = (dt(2024, 9, 1).date() - start.date()).days // 7
app = Flask(__name__)

# days_free = {
#     (dt(2024, 8, x) - start).days for x in range(16, 21)
# }

start_offset = start.weekday()

month_names = [
    "January", "February", "March", "April",
    "May", "June", "July", "August", "September",
    "October", "November", "December"
]


def calculate_day_delta_from_now() -> int:
    now = dt.now()
    return (now.date() - start.date()).days


@dataclass
class MonthData:
    offset: int = 0
    completed: int = 0
    remaining: int = 0

    @property
    def days(self) -> int:
        return self.completed + self.remaining


def generate_calendar():
    ret = {}
    end_date = end.date()
    current_date = start.date()
    day_delta_counter = calculate_day_delta_from_now()

    while current_date <= end_date:
        if current_date.year not in ret:
            ret[current_date.year] = {}

        if current_date.month not in ret[current_date.year]:
            ret[current_date.year][current_date.month] = MonthData(offset=current_date.weekday())

        md: MonthData = ret[current_date.year][current_date.month]

        if day_delta_counter > 0:
            md.completed += 1
            day_delta_counter -= 1
        else:
            md.remaining += 1

        current_date += timedelta(days=1)
    return ret


@app.route('/')
def main_route():
    calendar = generate_calendar()
    return render_template("index.html",
                           day_delta=calculate_day_delta_from_now(),
                           day_total=day_total,
                           # days_free=days_free,
                           start_offset=start_offset,
                           sbk_end=sbk_end,
                           calendar=calendar,
                           month_names=month_names
                           )


if __name__ == '__main__':
    app.run(debug=True)
