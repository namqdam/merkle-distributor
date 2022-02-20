import fs from 'fs';

import config from '../tests/config';

const { networkId, masterContractId } = config;

(function () {
  fs.copyFile(
    `${process.env.HOME}/.near-credentials/${networkId}/${masterContractId}.json`,
    `./neardev/${networkId}/${masterContractId}.json`,
    (err) => {
      if (err) throw err;
    }
  );

  fs.copyFile(
    `${process.env.HOME}/.near-credentials/${networkId}/ft.${masterContractId}.json`,
    `./neardev/${networkId}/ft.${masterContractId}.json`,
    (err) => {
      if (err) throw err;
    }
  );
})();
