const { join } = require("node:path");

const prefix = "/ahqstorebot";

/**
 * Handle
 * @param {import("@octokit/rest").Octokit} github
 * @param {{ payload: { comment: {body: string }, issue: { id: number, labels: {name: string}[] } }}} ctx
 */
module.exports = (github, ctx) => {
  const comment = ctx.payload.comment.body;
  const issueId = ctx.payload.issue.id;
  const labels = ctx.payload.issue.labels;
  const manifestPath = join(__dirname, "..", "..", "manifests");

  const base = {
    issue_number: issueId,
    owner: "ahqstore",
    repo: "apps",
  };

  const hasAppUpdateLabel = labels.includes(({ name }) => name == "app update");

  function createComment(body) {
    github.rest.issues.createComment({
      ...base,
      body,
    });
  }

  if (comment == `${prefix} -set-app-update`) {
    if (!hasAppUpdateLabel) {
      github.rest.issues.addLabels({
        ...base,
        labels: ["app update"],
      });
      createComment("✅ Labels Assigned");
    } else {
      createComment("❌ Labels Already Assigned");
    }
  } else if (comment.startsWith(`${prefix} /app-newConf-set`)) {
    if (hasAppUpdateLabel) {
      createComment(
        [
          `# 🚧 Under Development`,
          `IssueNumber: \`${issueId}\``,
          `Labels: \`${labels.map(({ name }) => name).join(", ")}\``,
          `ManifestPath: \`${manifestPath}\``,
        ].join("\n")
      );
    } else {
      createComment("❌ Labels Not Assigned");
    }
  }
};
