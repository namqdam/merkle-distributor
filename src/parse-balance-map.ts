import BalanceTree from './balance-tree';
import BN from 'bn.js';

export interface MerkleDistributorInfo {
  merkleRoot: string;
  tokenTotal: string;
  claims: {
    [account: string]: {
      index: number;
      amount: string;
      proof: string[];
      flags?: {
        [flag: string]: boolean;
      };
    };
  };
}

export type BalanceFormat = { address: string; earnings: string; reasons: string };

export function parseBalanceMap(balances: BalanceFormat[]): MerkleDistributorInfo {
  const dataByAddress = balances.reduce<{
    [address: string]: { amount: BN; flags?: { [flag: string]: boolean } };
  }>((memo, { address: account, earnings, reasons }) => {
    if (memo[account]) throw new Error(`Duplicate address: ${account}`);
    const parsedNum = new BN(earnings);
    if (parsedNum.lte(new BN(0))) throw new Error(`Invalid amount for account: ${account}`);

    const flags = {
      // isSOCKS: reasons.includes('socks'),
      // isLP: reasons.includes('lp'),
      // isUser: reasons.includes('user'),
    };

    memo[account] = { amount: parsedNum, ...(reasons === '' ? {} : { flags }) };
    return memo;
  }, {});

  const sortedAddresses = Object.keys(dataByAddress).sort();

  // construct a tree
  const tree = new BalanceTree(
    sortedAddresses.map((address) => ({ account: address, amount: dataByAddress[address].amount }))
  );

  // generate claims
  const claims = sortedAddresses.reduce<{
    [address: string]: { amount: string; index: number; proof: string[]; flags?: { [flag: string]: boolean } };
  }>((memo, address, index) => {
    const { amount, flags } = dataByAddress[address];
    memo[address] = {
      index,
      amount: amount.toString(),
      proof: tree.getProof(new BN(index), address, amount).map((proof) => proof.substring(2)),
      ...(flags ? { flags } : {})
    };
    return memo;
  }, {});

  const tokenTotal: BN = sortedAddresses.reduce<BN>((memo, key) => memo.add(dataByAddress[key].amount), new BN(0));

  return {
    merkleRoot: tree.getHexRoot().substring(2),
    tokenTotal: tokenTotal.toString(),
    claims
  };
}
