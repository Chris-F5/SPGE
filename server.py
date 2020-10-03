# This script is used for testing the game on localhost
import http.server
import socketserver

PORT = 8000

Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map.update({
     ".js": "application/javascript",
})

httpd = socketserver.TCPServer(("", PORT), Handler)
httpd.serve_forever()