const fs = require("fs");
const path = require("path");
const { parse } = require("node-html-parser");

const ROOT = path.resolve(__dirname, "..");
const DIST_DIR = path.join(ROOT, "dist");

const requiredPages = [
  "index.html",
  "docs/index.html",
  "docs/install/index.html",
  "docs/quickstart/index.html",
  "docs/commands/index.html",
  "docs/workflow/index.html",
  "docs/faq/index.html",
  "codex-power-user/index.html",
  "404.html",
  "go/install/index.html",
  "go/github/index.html"
];

const corePages = [
  "index.html",
  "docs/index.html",
  "docs/install/index.html",
  "docs/quickstart/index.html",
  "docs/commands/index.html",
  "docs/workflow/index.html",
  "docs/faq/index.html",
  "codex-power-user/index.html"
];

function collectHtmlFiles(dir) {
  const results = [];
  if (!fs.existsSync(dir)) {
    return results;
  }
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      results.push(...collectHtmlFiles(fullPath));
    } else if (entry.isFile() && entry.name.endsWith(".html")) {
      results.push(fullPath);
    }
  }
  return results;
}

function routeFromDistPath(filePath) {
  const rel = path.relative(DIST_DIR, filePath).split(path.sep).join("/");
  if (rel === "index.html") {
    return "/";
  }
  if (rel.endsWith("/index.html")) {
    return `/${rel.replace(/\/index\.html$/, "")}/`;
  }
  return `/${rel}`;
}

function isExternalHref(href) {
  return href.startsWith("http://") || href.startsWith("https://") || href.startsWith("mailto:") || href.startsWith("tel:") || href.startsWith("javascript:");
}

function resolveInternalLink(href, baseRoute) {
  const base = `http://local${baseRoute}`;
  let url;
  try {
    url = new URL(href, base);
  } catch (err) {
    return null;
  }
  const pathname = url.pathname;
  if (!pathname) {
    return null;
  }
  const cleaned = pathname.replace(/^\//, "");
  if (pathname === "/") {
    return path.join(DIST_DIR, "index.html");
  }
  if (path.extname(pathname)) {
    return path.join(DIST_DIR, cleaned);
  }
  if (pathname.endsWith("/")) {
    return path.join(DIST_DIR, cleaned, "index.html");
  }
  return path.join(DIST_DIR, cleaned, "index.html");
}

function runTests() {
  const errors = [];

  if (!fs.existsSync(DIST_DIR)) {
    errors.push("dist/ not found. Run `npm run build` first.");
    report(errors);
    process.exit(1);
  }

  for (const page of requiredPages) {
    const filePath = path.join(DIST_DIR, page);
    if (!fs.existsSync(filePath)) {
      errors.push(`Missing required page: dist/${page}`);
    }
  }

  for (const page of corePages) {
    const filePath = path.join(DIST_DIR, page);
    if (!fs.existsSync(filePath)) {
      continue;
    }
    const html = fs.readFileSync(filePath, "utf8");
    const root = parse(html);
    const primary = root.querySelector('[data-cta="primary"]');
    const secondary = root.querySelector('[data-cta="secondary"]');
    if (!primary) {
      errors.push(`Missing primary CTA in dist/${page}`);
    }
    if (!secondary) {
      errors.push(`Missing secondary CTA in dist/${page}`);
    }
  }

  const htmlFiles = collectHtmlFiles(DIST_DIR);
  for (const filePath of htmlFiles) {
    const html = fs.readFileSync(filePath, "utf8");
    const root = parse(html);
    const pageRoute = routeFromDistPath(filePath);
    const anchors = root.querySelectorAll("a");
    for (const anchor of anchors) {
      const href = anchor.getAttribute("href");
      if (!href || href === "#") {
        continue;
      }
      if (isExternalHref(href)) {
        continue;
      }
      const targetPath = resolveInternalLink(href, pageRoute);
      if (!targetPath) {
        continue;
      }
      if (!fs.existsSync(targetPath)) {
        errors.push(`Broken internal link: ${href} referenced from ${path.relative(DIST_DIR, filePath)}`);
      }
    }
  }

  report(errors);
  process.exit(errors.length ? 1 : 0);
}

function report(errors) {
  if (errors.length === 0) {
    console.log("All checks passed.");
    return;
  }
  console.error("Validation failed:");
  for (const err of errors) {
    console.error(`- ${err}`);
  }
}

runTests();
