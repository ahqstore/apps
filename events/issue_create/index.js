/**
 *
 * @param {import("@octokit/rest").Octokit} github
 * @param {{ payload: { comment: {body: string }, issue: { id: number } }}} ctx
 */
module.exports = (github, ctx) => {
  console.log(ctx, ctx.payload.comment.body, ctx.payload.issue.id);
};
