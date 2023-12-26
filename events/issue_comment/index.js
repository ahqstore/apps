/**
 * Handle
 * @param {import("@octokit/rest").Octokit} github
 * @param {{ payload: { comment: {body: string }, issue: { number: number, id: number, labels: {name: string}[] } }}} ctx
 */
module.exports = (github, ctx) => {
  console.log(JSON.stringify(ctx, null, 2));
};
