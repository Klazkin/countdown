from flask import Flask, send_from_directory
import os
import mimetypes

app = Flask(__name__,
            static_url_path='',
            static_folder='static')

mimetypes.add_type('application/wasm', '.wasm')

@app.route('/')
def serve_index():
    return send_from_directory(app.static_folder, 'index.html')

if __name__ == '__main__':
    app.run()