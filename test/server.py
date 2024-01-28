#!/usr/bin/env python
from argparse import ArgumentParser
from http.server import HTTPServer, SimpleHTTPRequestHandler
import os


parser = ArgumentParser()
parser.add_argument("-d", "--directory")
args = parser.parse_args()

if args.directory is not None:
    os.chdir(args.directory)

SimpleHTTPRequestHandler.extensions_map.update({
    '.wasm': 'application/wasm',
})

server_address = ('', 8000)
with HTTPServer(server_address, SimpleHTTPRequestHandler) as httpd:
    print("Ready!")
    httpd.serve_forever()
