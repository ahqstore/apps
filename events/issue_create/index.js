/**
 *
 * @param {import("@octokit/rest").Octokit} github
 * @param {{ payload: { comment: {body: string }, issue: { id: number, labels: string[] } }}} ctx
 */
module.exports = (github, ctx) => {
  console.log(
    ctx.payload.comment.body,
    ctx.payload.issue.id,
    ctx.payload.issue.labels
  );
};
