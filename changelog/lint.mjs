import changelog from "remark-changelog";
import { readSync, writeSync } from "to-vfile";
import * as remark from "remark";
import { reporter } from "vfile-reporter";

const file = await remark
  .remark()
  .use(() => changelog({ repository: "https://github.com/strowk/tisq" }))
  .process(readSync("CHANGELOG.md"));

console.error(reporter(file));
if (file.messages.length > 0) {
  process.exit(1);
}
