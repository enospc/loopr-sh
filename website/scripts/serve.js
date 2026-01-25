const http = require("http");
const fs = require("fs");
const path = require("path");

const MIME_TYPES = {
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".ico": "image/x-icon"
};

function resolveFile(rootDir, urlPath) {
  const cleanPath = urlPath.split("?")[0].split("#")[0];
  if (cleanPath === "/") {
    return path.join(rootDir, "index.html");
  }
  const absolute = path.join(rootDir, cleanPath.replace(/^\//, ""));
  if (fs.existsSync(absolute) && fs.statSync(absolute).isFile()) {
    return absolute;
  }
  if (!path.extname(absolute)) {
    const withIndex = path.join(absolute, "index.html");
    if (fs.existsSync(withIndex)) {
      return withIndex;
    }
    const withHtml = `${absolute}.html`;
    if (fs.existsSync(withHtml)) {
      return withHtml;
    }
  }
  const fallback404 = path.join(rootDir, "404.html");
  if (fs.existsSync(fallback404)) {
    return fallback404;
  }
  return null;
}

function startServer(rootDir, port = 3000) {
  const server = http.createServer((req, res) => {
    const filePath = resolveFile(rootDir, req.url || "/");
    if (!filePath) {
      res.statusCode = 404;
      res.end("Not found");
      return;
    }
    const ext = path.extname(filePath);
    res.setHeader("Content-Type", MIME_TYPES[ext] || "application/octet-stream");
    const status = filePath.endsWith("404.html") ? 404 : 200;
    res.statusCode = status;
    res.end(fs.readFileSync(filePath));
  });

  server.listen(port, () => {
    console.log(`Serving ${rootDir} at http://localhost:${port}`);
  });

  return server;
}

module.exports = { startServer };
