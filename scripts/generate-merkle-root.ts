import { program } from 'commander';
import fs from 'fs';
import path from 'path';
import { parseBalanceMap } from '../src/parse-balance-map';

(function () {
  program
    .version('0.0.0')
    .option('-o, --output <path>', 'ouput JSON file location for saving')
    .requiredOption(
      '-i, --input <path>',
      'input JSON file location containing a map of account addresses to string balances'
    );

  program.parse(process.argv);

  const inputPath = program.opts().i || program.opts().input;
  if (!inputPath) return;

  const json = JSON.parse(fs.readFileSync(path.resolve(inputPath), { encoding: 'utf8' }));

  if (typeof json !== 'object') throw new Error('Invalid JSON');

  const result = parseBalanceMap(json);
  console.log(JSON.stringify(result));

  const outputPath = program.opts().o || program.opts().output;
  if (!outputPath) return;

  fs.writeFileSync(path.resolve(outputPath), JSON.stringify(parseBalanceMap(json), null, 2));
})();
