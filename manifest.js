const { mkdirSync, writeFileSync } = require("fs");

mkdirSync("./manifests");

// From 'A' upto 'Z' loop
for (let i = 65; i <= 90; i++) {
  const char = String.fromCharCode(i).toLowerCase();

  mkdirSync(`./manifests/${char}`);
  writeFileSync(`./manifests/${char}/ignore`, "ignore this file");
}
