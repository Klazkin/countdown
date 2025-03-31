from flask import Flask, send_from_directory
import os
import mimetypes

app = Flask(__name__, static_folder='dist')

mimetypes.add_type('application/wasm', '.wasm')

@app.route('/')
def serve_index():
    return send_from_directory(app.static_folder, 'index.html')

@app.route('/<path:filename>')
def serve_static(filename):
    return send_from_directory(app.static_folder, filename)

if __name__ == '__main__':
    app.run(debug=True)