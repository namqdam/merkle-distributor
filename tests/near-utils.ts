import fs from 'fs';
import { Account, Contract, KeyPair, Near } from 'near-api-js';
import { InMemoryKeyStore } from 'near-api-js/lib/key_stores';

import config from './config';

const { masterContractId, nodeUrl, networkId } = config;

function prepare(networkId: string, contractId: string) {
  const credPath = `./neardev/${networkId}/${contractId}.json`;
  console.log('Loading Credentials:\n', credPath);

  let credentials;
  try {
    credentials = JSON.parse(fs.readFileSync(credPath, { encoding: 'utf8' }));
  } catch (e) {
    console.warn(e);
    /// attempt to load backup creds from local machine
    credentials = JSON.parse(
      fs.readFileSync(`${process.env.HOME}/.near-credentials/${networkId}/${contractId}.json`, { encoding: 'utf8' })
    );
  }
  return credentials;
}

function init() {
  const ftContractId = `ft.${masterContractId}`;

  const keyStore = new InMemoryKeyStore();

  const masterCredentials = prepare(networkId, masterContractId);
  const ftCredentials = prepare(networkId, ftContractId);

  keyStore.setKey(networkId, masterContractId, KeyPair.fromString(masterCredentials.private_key));
  keyStore.setKey(networkId, ftContractId, KeyPair.fromString(ftCredentials.private_key));

  const near = new Near({
    networkId,
    nodeUrl,
    keyStore,
    headers: {}
  });

  const masterAccount = new Account(near.connection, masterContractId);
  const ftAccount = new Account(near.connection, ftContractId);

  const masterContract = new Contract(masterAccount, masterAccount.accountId, {
    changeMethods: ['initialize'],
    viewMethods: []
  });

  const ftContract = new Contract(ftAccount, ftAccount.accountId, {
    changeMethods: ['new_default_meta'],
    viewMethods: []
  });

  return {
    masterAccount,
    ftAccount,
    masterContract,
    ftContract,
    config,
    near,
    keyStore
  };
}

export default init();
