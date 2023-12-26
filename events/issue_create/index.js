/**
 *
 * @param {import("@octokit/rest").Octokit} github
 * @param {{ payload: { body: string }, issue: { id: number } }} ctx
 */
module.exports = (github, ctx) => {
  console.log(ctx.payload.body, ctx.issue.id);
};
