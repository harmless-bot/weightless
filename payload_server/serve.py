#!/usr/bin/env python3
import os
import http.server
import socketserver

class CustomHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        # Serve from ../test_payloads/ relative to this script
        script_dir = os.path.dirname(os.path.abspath(__file__))
        directory = os.path.join(script_dir, "..", "test_payloads")
        super().__init__(*args, directory=directory, **kwargs)

    def log_message(self, format, *args):
        try:
            requestline, code, size = args
            req_parts = requestline.split()
            method = req_parts[0] if len(req_parts) > 0 else "GET"
            path = req_parts[1] if len(req_parts) > 1 else self.path
        except Exception:
            method = "GET"
            path = self.path
            code = "-"
            size = "-"
        
        client = self.client_address[0]
        print(f"[REQ] {method} {path} from {client} — {code} ({size} bytes)")

def main():
    PORT = 8080
    Handler = CustomHandler

    try:
        with socketserver.TCPServer(("127.0.0.1", PORT), Handler) as httpd:
            print("[*] Weightless payload server")
            print(f"[*] Listening: http://127.0.0.1:{PORT}")
            print("[*] Serving:   ../test_payloads/")
            print("[*] CTRL+C to stop")
            httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n[*] Stopped")

if __name__ == "__main__":
    main()
