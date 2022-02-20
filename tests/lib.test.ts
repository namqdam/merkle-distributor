import { BN } from 'bn.js';
import { Account, Contract } from 'near-api-js';
import { parseNearAmount } from 'near-api-js/lib/utils/format';

import { BalanceFormat, MerkleDistributorInfo } from '../src/parse-balance-map';
import testUtils from './test-utils';

const GAS = new BN('200000000000000');

jest.setTimeout(300_000);

describe('Deploy contracts', () => {
  let masterInfo: {
    contract: Contract;
    account: Account;
    balances: BalanceFormat[];
    merkle: MerkleDistributorInfo;
  };

  let ftInfo: {
    contract: Contract;
    account: Account;
  };

  beforeAll(async () => {
    const { master, ft } = await testUtils.initContracts();
    masterInfo = master;
    ftInfo = ft;
  });

  test('Simple merkle', async () => {
    const claim = masterInfo.merkle.claims[ftInfo.account.accountId];

    await ftInfo.account.functionCall({
      contractId: ftInfo.contract.contractId,
      methodName: 'storage_deposit',
      args: { account_id: masterInfo.contract.contractId },
      gas: GAS,
      attachedDeposit: new BN(parseNearAmount('0.1') ?? '0')
    });

    await ftInfo.account.functionCall({
      contractId: ftInfo.contract.contractId,
      methodName: 'ft_transfer_call',
      args: {
        receiver_id: masterInfo.contract.contractId,
        amount: masterInfo.merkle.tokenTotal,
        msg: 'deposit-for-claims'
      },
      gas: GAS,
      attachedDeposit: new BN('1')
    });

    await ftInfo.account.functionCall({
      contractId: masterInfo.contract.contractId,
      methodName: 'claim',
      args: { index: `${claim.index}`, amount: claim.amount, proof: claim.proof },
      gas: GAS
    });

    const tokenBalance = await ftInfo.account.viewFunction(ftInfo.contract.contractId, 'ft_balance_of', {
      account_id: ftInfo.account.accountId
    });

    const tokenClaimed = new BN(tokenBalance).add(new BN(masterInfo.merkle.tokenTotal)).sub(new BN('1000000000'));

    const claimedAmount = await ftInfo.account.viewFunction(masterInfo.contract.contractId, 'get_claimed_amount', {
      account_id: ftInfo.account.accountId
    });

    expect(new BN(claimedAmount).eq(new BN(claim.amount))).toEqual(true);
    expect(new BN(claimedAmount).eq(tokenClaimed)).toEqual(true);
  });
});
