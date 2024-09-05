const { exec } = require("child_process");
const { join } = require("path");

const workspace = join(__dirname, "../../");

exec(
  "cargo run",
  {
    cwd: workspace,
    env: {
      ...process.env,
      RUSTFLAGS: "-Awarnings",
    },
  },
  console.log
);
