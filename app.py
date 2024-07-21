from flask import Flask, render_template
from datetime import datetime

start = datetime(2024, 7, 16, 12, 30, 0)
now = datetime.now()
end = datetime(2025, 6, 20, 12)
day_total = (end - start).days
day_delta = (now - start).days
app = Flask(__name__)


@app.route('/')
def main_route():
    return render_template("index.html", day_delta=day_delta, day_total=day_total)


if __name__ == '__main__':
    app.run()
