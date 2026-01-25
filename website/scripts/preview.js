const path = require("path");
const { startServer } = require("./serve");

const ROOT = path.resolve(__dirname, "..");
const PORT = Number(process.env.PORT || 5000);

startServer(path.join(ROOT, "dist"), PORT);
