const fs = require("fs");
const path = require("path");
const matter = require("gray-matter");
const MarkdownIt = require("markdown-it");

const ROOT = path.resolve(__dirname, "..");
const REPO_ROOT = path.resolve(ROOT, "..");
const CONTENT_DIR = path.join(ROOT, "content");
const DIST_DIR = path.join(ROOT, "dist");
const ASSETS_DIR = path.join(ROOT, "assets");
const TEMPLATES_DIR = path.join(ROOT, "templates");
const CONFIG_PATH = path.join(ROOT, "site.config.json");

function readConfig() {
  const raw = fs.readFileSync(CONFIG_PATH, "utf8");
  const config = JSON.parse(raw);
  const envToken =
    process.env.CF_ANALYTICS_TOKEN ||
    process.env.CLOUDFLARE_ANALYTICS_TOKEN ||
    "";
  const analyticsToken = envToken || config.analytics.cloudflareToken || "__CF_ANALYTICS_TOKEN__";
  return { ...config, analyticsToken };
}

function renderTemplate(template, vars) {
  return template.replace(/{{\s*([a-zA-Z0-9_]+)\s*}}/g, (match, key) => {
    if (Object.prototype.hasOwnProperty.call(vars, key)) {
      return String(vars[key]);
    }
    return "";
  });
}

function ensureDir(dir) {
  fs.mkdirSync(dir, { recursive: true });
}

function removeDir(dir) {
  if (fs.existsSync(dir)) {
    fs.rmSync(dir, { recursive: true, force: true });
  }
}

function copyDir(src, dest) {
  if (!fs.existsSync(src)) {
    return;
  }
  ensureDir(dest);
  for (const entry of fs.readdirSync(src, { withFileTypes: true })) {
    const srcPath = path.join(src, entry.name);
    const destPath = path.join(dest, entry.name);
    if (entry.isDirectory()) {
      copyDir(srcPath, destPath);
    } else {
      fs.copyFileSync(srcPath, destPath);
    }
  }
}

function collectMarkdownFiles(dir) {
  const results = [];
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      results.push(...collectMarkdownFiles(fullPath));
    } else if (entry.isFile() && entry.name.endsWith(".md")) {
      results.push(fullPath);
    }
  }
  return results;
}

function normalizePath(p) {
  if (!p.startsWith("/")) {
    return `/${p}`;
  }
  return p;
}

function ensureTrailingSlash(p) {
  if (p === "/") {
    return "/";
  }
  return p.endsWith("/") ? p : `${p}/`;
}

function isWithinRoot(targetPath, rootPath) {
  const normalizedRoot = path.resolve(rootPath);
  const normalizedTarget = path.resolve(targetPath);
  if (normalizedTarget === normalizedRoot) {
    return true;
  }
  return normalizedTarget.startsWith(`${normalizedRoot}${path.sep}`);
}

function loadMarkdownContent(filePath, parsed, frontMatter) {
  if (!frontMatter.source) {
    return parsed.content || "";
  }
  const sourcePath = path.resolve(ROOT, frontMatter.source);
  if (!isWithinRoot(sourcePath, REPO_ROOT)) {
    throw new Error(`Content source outside repo root: ${frontMatter.source}`);
  }
  if (!fs.existsSync(sourcePath)) {
    throw new Error(`Content source not found: ${frontMatter.source}`);
  }
  return fs.readFileSync(sourcePath, "utf8");
}

function routeFromFile(relPath, frontMatter) {
  if (frontMatter.permalink) {
    return frontMatter.permalink;
  }
  const parts = relPath.split(path.sep);
  const file = parts.pop();
  const name = file.replace(/\.md$/, "");
  if (name === "index") {
    const dirPath = parts.join("/");
    if (!dirPath) {
      return "/";
    }
    return ensureTrailingSlash(`/${dirPath}`);
  }
  const dirPath = parts.join("/");
  const slugPath = dirPath ? `/${dirPath}/${name}` : `/${name}`;
  return ensureTrailingSlash(slugPath);
}

function outputPathFromRoute(route) {
  if (route.endsWith(".html")) {
    const cleaned = route.replace(/^\/+/, "");
    return path.join(DIST_DIR, cleaned);
  }
  const normalized = ensureTrailingSlash(route);
  if (normalized === "/") {
    return path.join(DIST_DIR, "index.html");
  }
  const trimmed = normalized.replace(/^\//, "");
  return path.join(DIST_DIR, trimmed, "index.html");
}

function buildNav(navItems, currentPath) {
  const current = ensureTrailingSlash(normalizePath(currentPath));
  return navItems
    .map((item) => {
      const href = item.href;
      const isExternal = href.startsWith("http");
      const normalizedHref = ensureTrailingSlash(normalizePath(href));
      let active = "";
      if (!isExternal) {
        if (normalizedHref === "/" && current === "/") {
          active = " active";
        } else if (normalizedHref !== "/" && current.startsWith(normalizedHref)) {
          active = " active";
        }
      }
      return `<li><a class="nav-link${active}" href="${href}">${item.label}</a></li>`;
    })
    .join("\n");
}

function buildFooter(navItems, ctas) {
  const links = [
    { label: "Install", href: ctas.primary.href },
    { label: "Docs", href: ctas.secondary.href },
    { label: "GitHub", href: "/go/github/" }
  ];
  const extra = navItems
    .filter((item) => !links.find((link) => link.href === item.href))
    .slice(0, 3);
  return [...links, ...extra]
    .map((item) => `<a href="${item.href}">${item.label}</a>`)
    .join("\n");
}

function analyticsSnippet(token) {
  return `\n<script defer src="https://static.cloudflareinsights.com/beacon.min.js" data-cf-beacon='{"token":"${token}"}'></script>`;
}

function buildSite() {
  const config = readConfig();
  const layout = fs.readFileSync(path.join(TEMPLATES_DIR, "layout.html"), "utf8");
  const redirectLayout = fs.readFileSync(path.join(TEMPLATES_DIR, "redirect.html"), "utf8");
  const md = new MarkdownIt({ html: true, linkify: true });

  removeDir(DIST_DIR);
  ensureDir(DIST_DIR);
  copyDir(ASSETS_DIR, path.join(DIST_DIR, "assets"));

  const files = collectMarkdownFiles(CONTENT_DIR);
  const year = new Date().getFullYear();

  for (const filePath of files) {
    const relPath = path.relative(CONTENT_DIR, filePath);
    const raw = fs.readFileSync(filePath, "utf8");
    const parsed = matter(raw);
    const frontMatter = parsed.data || {};
    const route = routeFromFile(relPath, frontMatter);
    const outputPath = outputPathFromRoute(route);
    const titleText = frontMatter.title || config.site.name;
    const pageTitle = frontMatter.title ? `${frontMatter.title} | ${config.site.name}` : config.site.name;
    const description = frontMatter.description || config.site.description;
    const ogTitle = frontMatter.ogTitle || pageTitle;
    const ogDescription = frontMatter.ogDescription || description;
    const ogImage = frontMatter.ogImage || `${config.site.baseUrl}/assets/og-default.png`;
    const ogUrl = `${config.site.baseUrl}${route === "/" ? "" : route}`;
    const navItems = buildNav(config.nav, route);
    const footerItems = buildFooter(config.nav, config.ctas);
    const commonVars = {
      lang: frontMatter.lang || "en",
      title: pageTitle,
      description,
      ogTitle,
      ogDescription,
      ogImage,
      ogUrl,
      twitterCard: "summary_large_image",
      tagline: config.site.tagline,
      navItems,
      footerItems,
      ctaPrimaryLabel: config.ctas.primary.label,
      ctaPrimaryHref: config.ctas.primary.href,
      ctaSecondaryLabel: config.ctas.secondary.label,
      ctaSecondaryHref: config.ctas.secondary.href,
      year,
      siteName: config.site.name,
      bodyClass: frontMatter.bodyClass || "",
      analyticsSnippet: analyticsSnippet(config.analyticsToken),
      extraHead: frontMatter.extraHead || ""
    };

    ensureDir(path.dirname(outputPath));

    if (frontMatter.redirect) {
      const redirectHtml = renderTemplate(redirectLayout, {
        ...commonVars,
        title: titleText || "Redirecting",
        redirectUrl: frontMatter.redirect
      });
      fs.writeFileSync(outputPath, redirectHtml);
      continue;
    }

    const contentHtml = md.render(loadMarkdownContent(filePath, parsed, frontMatter));
    const pageHtml = renderTemplate(layout, {
      ...commonVars,
      content: contentHtml
    });
    fs.writeFileSync(outputPath, pageHtml);
  }

  const ogFallbackPath = path.join(DIST_DIR, "assets", "og-default.png");
  if (!fs.existsSync(ogFallbackPath)) {
    const placeholder = Buffer.from(
      "iVBORw0KGgoAAAANSUhEUgAAAZAAAAGQCAIAAAC8u5S0AAAAA3NCSVQICAjb4U/gAAABFElEQVR4nO3BMQEAAADCoPVPbQ0PoAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD4B1N9AAE9YSu9AAAAAElFTkSuQmCC",
      "base64"
    );
    ensureDir(path.dirname(ogFallbackPath));
    fs.writeFileSync(ogFallbackPath, placeholder);
  }

  return { files: files.length };
}

if (require.main === module) {
  const result = buildSite();
  console.log(`Built ${result.files} content files.`);
}

module.exports = { buildSite };
