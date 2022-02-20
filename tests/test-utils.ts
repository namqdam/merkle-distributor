import BN from 'bn.js';
import { nanoid } from 'nanoid';

import { BalanceFormat, parseBalanceMap } from '../src/parse-balance-map';
import nearInfo from './near-utils';

const { masterAccount, ftAccount, masterContract, ftContract, config, near, keyStore } = nearInfo;
const GAS = new BN('200000000000000');

function randomMerkle(contractId: string) {
  const rndInt = Math.floor(Math.random() * 10) + 1;
  const balances: BalanceFormat[] = [];

  for (let index = 0; index < 10; index++) {
    if (index === rndInt) {
      balances.push({
        address: contractId,
        earnings: `${index * 1000}`,
        reasons: ''
      });
    } else {
      balances.push({
        address: nanoid(10),
        earnings: `${(index + 1) * 1000}`,
        reasons: ''
      });
    }
  }
  return { balances, merkle: parseBalanceMap(balances) };
}

async function initContracts() {
  const { balances, merkle } = randomMerkle(ftAccount.accountId);

  /// try to call new on contract, swallow e if already initialized
  try {
    const args = {
      owner_id: ftAccount.accountId,
      total_supply: '1000000000'
    };
    await ftAccount.functionCall({
      contractId: ftContract.contractId,
      methodName: 'new_default_meta',
      args,
      gas: GAS
    });
  } catch (e: any) {
    if (!/initialized/.test(e.toString())) {
      throw e;
    }
  }

  /// try to call new on contract, swallow e if already initialized
  try {
    const args = {
      owner_id: masterAccount.accountId,
      token_id: ftContract.contractId,
      merkle_root: merkle.merkleRoot
    };
    await masterAccount.functionCall({
      contractId: masterContract.contractId,
      methodName: 'initialize',
      args,
      gas: GAS
    });
  } catch (e: any) {
    if (!/initialized/.test(e.toString())) {
      throw e;
    }
  }

  return {
    master: {
      contract: masterContract,
      account: masterAccount,
      balances,
      merkle
    },
    ft: {
      contract: ftContract,
      account: ftAccount
    }
  };
}

export default {
  config,
  near,
  keyStore,
  masterAccount,
  ftAccount,
  initContracts
};
