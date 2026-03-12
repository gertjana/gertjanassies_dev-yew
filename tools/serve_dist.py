#!/usr/bin/env python3
"""
Minimal static file server that replicates Nginx's try_files behaviour:
  try_files $uri $uri/index.html /index.html

Used by `make serve-dist` to test the generated dist/ locally, including
the meta-page sidecars at dist/post/{slug}/index.html.

Usage: python3 tools/serve_dist.py [port] [dist_dir]
"""

import http.server
import pathlib
import sys


DIST_DIR = pathlib.Path(sys.argv[2]) if len(sys.argv) > 2 else pathlib.Path("dist")
PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8080


class TryFilesHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(DIST_DIR), **kwargs)

    def do_GET(self):
        # Strip query string, remove leading slash
        rel = self.path.split("?")[0].lstrip("/")

        # try_files: $uri, $uri/index.html, /index.html (SPA fallback)
        candidates = [
            DIST_DIR / rel,
            DIST_DIR / rel / "index.html",
            DIST_DIR / "index.html",
        ]

        for candidate in candidates:
            if candidate.is_file():
                self.path = "/" + str(candidate.relative_to(DIST_DIR))
                break

        return super().do_GET()

    def log_message(self, format, *args):  # noqa: A002
        print(format % args)


if __name__ == "__main__":
    if not DIST_DIR.exists():
        print(f"Error: dist directory not found: {DIST_DIR}")
        print("Run `make build-frontend-web` first.")
        sys.exit(1)

    print(f"Serving {DIST_DIR.resolve()} on http://localhost:{PORT}")
    print("Real files are served directly; unknown paths fall back to index.html.")
    print("Press Ctrl+C to stop.\n")

    server = http.server.HTTPServer(("", PORT), TryFilesHandler)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nStopped.")
