#!/usr/bin/env python
from http.server import HTTPServer, SimpleHTTPRequestHandler


SimpleHTTPRequestHandler.extensions_map.update({
    '.wasm': 'application/wasm',
})

server_address = ('', 8000)
with HTTPServer(server_address, SimpleHTTPRequestHandler) as httpd:
    print("Ready!")
    httpd.serve_forever()
