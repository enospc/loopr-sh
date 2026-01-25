const path = require("path");
const chokidar = require("chokidar");
const { buildSite } = require("./build");
const { startServer } = require("./serve");

const ROOT = path.resolve(__dirname, "..");
const CONTENT_DIR = path.join(ROOT, "content");
const TEMPLATES_DIR = path.join(ROOT, "templates");
const ASSETS_DIR = path.join(ROOT, "assets");
const CONFIG_PATH = path.join(ROOT, "site.config.json");
const PORT = Number(process.env.PORT || 3000);

let building = false;
let pending = false;

function rebuild() {
  if (building) {
    pending = true;
    return;
  }
  building = true;
  try {
    const result = buildSite();
    console.log(`Rebuilt ${result.files} content files.`);
  } catch (err) {
    console.error("Build failed:", err);
  } finally {
    building = false;
    if (pending) {
      pending = false;
      rebuild();
    }
  }
}

buildSite();
startServer(path.join(ROOT, "dist"), PORT);

const watcher = chokidar.watch([CONTENT_DIR, TEMPLATES_DIR, ASSETS_DIR, CONFIG_PATH], {
  ignoreInitial: true
});

watcher.on("all", () => {
  rebuild();
});
