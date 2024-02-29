/**
 * Handle
 * @param {import("@octokit/rest").Octokit} github
 * @param {{ payload: { comment: {body: string }, issue: { number: number, id: number, labels: {name: string}[] } }}} ctx
 */
module.exports = async (github, ctx) => {
  console.log(JSON.stringify(ctx, null, 4));
  const issue = ctx.payload.issue.id;

  if (issue == 3) {
    const data = await github.issues.createComment({});
  }
};
