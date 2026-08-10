/**
 * Runs a local box through the typed Node consumer.
 *
 * SETUP (once):
 *   npm install scrollcase
 *   npm install --save-dev tsx typescript
 *
 * RUN:
 *   npx tsx consumer-templates/run-box.ts
 *
 * Replace <target> and <hash> below with the values printed by scrollcase build.
 */
import { runBox } from 'scrollcase/consumer';

const releaseToRun =
  '.scrollcase/dist/boxes/example-box/1.0.0/<target>/<hash>.release.json';

runBox(releaseToRun, {
  publicPath: '.scrollcase/keys/signing-public.json',
  args: [],
  stdin: 'inherit',
  stdout: 'inherit',
  stderr: 'inherit',
  onPrepared: ({ boxId, version, targetId }) => {
    console.log(`Running ${boxId} ${version} (${targetId})`);
  },
}).then((result) => {
  if (result.signal) console.error(`Box exited after ${result.signal}.`);
  process.exitCode = result.exitCode ?? 1;
});
