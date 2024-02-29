//@ts-check
const { exec } = require("child_process");
const { writeFileSync } = require("fs");
const { join } = require("path");

/**
 * Handle
 * @param {import("@octokit/rest").Octokit} github
 * @param {{ payload: { comment: { number: number, id: number, body: string, user: { login: string } }, issue: { number: number, id: number, labels: {name: string}[] } }}} ctx
 */
module.exports = async (github, ctx) => {
  console.log(JSON.stringify(ctx, null, 4));
  const issue = ctx.payload.issue.id;
  const issue_number = ctx.payload.issue.number;

  const author_username = ctx.payload.comment.user.login;

  const body = ctx.payload.comment.body;

  const [slash, cmd, link] = body.split(" ");

  const owner = "ahqstore";
  const repo = "apps";

  if (issue == 2161399623 && slash == "/store") {
    if (cmd == "set") {
      const req = await fetch(link).then((req) => req.text());
      writeFileSync("./bytes.txt", `${owner}&${req}`);

      const workspace = join(__dirname, "../../");
      exec(
        "cargo run --no-default-features --features load_bytes",
        {
          cwd: workspace,
        },
        async (err, out, stderr) => {
          const body = `@${author_username}
# Output
${out}
# Error
${err}
${stderr}`;

          console.log(body);

          await github.rest.issues.createComment({
            body,
            owner,
            repo,
            issue_number,
          });
        }
      );
    } else if (cmd == "remove") {
      await github.rest.issues.createComment({
        body: "Under Work",
        owner,
        repo,
        issue_number,
      });
    }
  }
};