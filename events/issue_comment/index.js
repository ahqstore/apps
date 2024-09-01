//@ts-check
const { exec } = require("child_process");
const { writeFileSync, readFileSync, rmSync, existsSync } = require("fs");
const { join } = require("path");

/**
 * Handle
 * @param {import("@octokit/rest").Octokit} github
 * @param {{ payload: { comment: { number: number, id: number, body: string, user: { login: string } }, issue: { number: number, id: number, labels: {name: string}[] } }}} ctx
 */
module.exports = async (github, ctx) => {
  const issue = ctx.payload.issue.id;
  const issue_number = ctx.payload.issue.number;

  const author_username = ctx.payload.comment.user.login;

  const body = ctx.payload.comment.body;

  const [slash, cmd, link] = body.split(" ");

  const owner = "ahqstore";
  const repo = "apps";

  if (issue != 2161399623) {
    return;
  }

  if (slash == "/store") {
    if (cmd == "set") {
      const req = await fetch(link).then((req) => req.text());
      console.log(req);
      const bytes = new TextEncoder().encode(req);

      writeFileSync("./bytes.txt", `${author_username}&[${bytes}]`);

      const workspace = join(__dirname, "../../");
      exec(
        "cargo run --features load_bytes",
        {
          cwd: workspace,
          env: {
            ...process.env,
            RUSTFLAGS: "-Awarnings",
          },
        },
        async (err, out, stderr) => {
          const body = `@${author_username}
# Console
\`\`\`
${out}
\`\`\`
# IO ERR
\`\`\`
${stderr}

--- ERR ---
${String(err) == String(stderr) ? "None" : err}
\`\`\``;

          console.log(body);

          await github.rest.issues.createComment({
            body,
            owner,
            repo,
            issue_number,
          });

          return out.includes("Successful");
        }
      );
    } else if (cmd == "remove") {
      writeFileSync(
        "./bytes.txt",
        `./manifests/${author_username[0]}/${author_username}/${link}.json`
      );

      const workspace = join(__dirname, "../../");
      exec(
        "cargo run --features remove_manifest",
        {
          cwd: workspace,
          env: {
            ...process.env,
            RUSTFLAGS: "-Awarnings",
          },
        },
        async (err, out, stderr) => {
          const body = `@${author_username}
# Console
\`\`\`
${out || "No Output"}
\`\`\`
# IO
\`\`\`
${err || "No Error"}
${stderr || "No StdErr Terminal"}
\`\`\``;

          console.log(body);

          await github.rest.issues.createComment({
            body,
            owner,
            repo,
            issue_number,
          });

          return out.includes("Successful");
        }
      );
    }
  } else if (slash == "/account") {
    if (cmd == "create" || cmd == "mutate") {
      const path = join(__dirname, `../../users/${author_username}.json`);

      if (existsSync(path) && cmd == "create") {
        await github.rest.issues.createComment({
          body: "User already exists, use /account mutate <path to acc file>",
          owner,
          repo,
          issue_number,
        });
        return;
      }

      const acc = await fetch(link).then((s) => s.json());

      acc.github = author_username;
      acc.avatar_url = null;

      writeFileSync(path, JSON.stringify(acc, null, 2));
    } else if (cmd == "remove") {
      const user = link;

      if (user != author_username) {
        await github.rest.issues.createComment({
          body: "You can only remove your own account",
          owner,
          repo,
          issue_number,
        });
        return;
      }
      const path = join(__dirname, `../../users/${author_username}.json`);

      rmSync(path);
    }
  }
};
