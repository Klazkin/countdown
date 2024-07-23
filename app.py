from flask import Flask, render_template
from datetime import datetime

dt = datetime

start = dt(2024, 7, 16, 12, 30, 0)
now = dt.now()
end = dt(2025, 6, 20, 12)
day_total = (end - start).days
day_delta = (now - start).days
sbk_end = (dt(2024, 9, 1) - start).days // 7
app = Flask(__name__)

days_free = [
    (dt(2024, 8, x) - start).days for x in range(16, 21)
]

start_offset = start.weekday()

@app.route('/')
def main_route():
    return render_template("index.html",
                           day_delta=day_delta,
                           day_total=day_total,
                           days_free=days_free,
                           start_offset=start_offset,
                           sbk_end=sbk_end
                           )


if __name__ == '__main__':
    app.run(debug=True)
