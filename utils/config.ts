import { execSync } from 'child_process';
import fs from 'fs';
import https from 'https';
import path from 'path';

function downloadFT() {
  return new Promise<void>((resolve, reject) => {
    const ftPath =
      'https://raw.githubusercontent.com/near/near-sdk-rs/master/examples/fungible-token/res/fungible_token.wasm';
    const dest = path.resolve('res', 'fungible_token.wasm');

    const file = fs.createWriteStream(dest);

    https
      .get(ftPath, (response) => {
        response.pipe(file);
        file.on('finish', () => {
          file.close();
          resolve();
        });
      })
      .on('error', (err) => {
        // Handle errors
        fs.unlink(dest, () => {
          reject(err.message);
        });
      });
  });
}

function deploy() {
  execSync('near dev-deploy --wasmFile res/merkle_distributor.wasm', { stdio: 'inherit' });
  const contractId = fs.readFileSync('./neardev/dev-account').toString();
  const ftContractId = `ft.${contractId}`;
  execSync(`near create-account ${ftContractId} --initialBalance 10 --masterAccount ${contractId}`, {
    stdio: 'inherit'
  });
  execSync(`near deploy --accountId ${ftContractId} --wasmFile res/fungible_token.wasm`, { stdio: 'inherit' });
}

function editConfig() {
  const contractId = fs.readFileSync('./neardev/dev-account').toString();
  const utilPath = './tests/config.ts';

  let data = fs.readFileSync(utilPath, { encoding: 'utf-8' });
  data = data.replace(/.*const masterContractId.*/gim, `const masterContractId = '${contractId}';`);

  fs.writeFileSync(utilPath, data, { encoding: 'utf-8' });
}

(async function () {
  await downloadFT();
  deploy();
  editConfig();
})();
