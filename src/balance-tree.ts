import keccak256 from 'keccak256';
import MerkleTree from 'merkletreejs';
import BN from 'bn.js';

export default class BalanceTree {
  private readonly tree: MerkleTree;

  constructor(balances: { account: string; amount: BN }[]) {
    this.tree = new MerkleTree(
      balances.map(({ account, amount }, index) => {
        return BalanceTree.toNode(new BN(index), account, amount);
      }),
      keccak256,
      {
        sort: true
      }
    );
  }

  public static verifyProof(index: BN, account: string, amount: BN, proof: Buffer[], root: Buffer): boolean {
    let pair = BalanceTree.toNode(index, account, amount);
    return MerkleTree.verify(proof, pair, root, keccak256);
  }

  public static toNode(index: BN, account: string, amount: BN): Buffer {
    let _index = new BN(index).toBuffer('le', 8);
    let _account = Buffer.from(account, 'utf-8');
    let _amount = new BN(amount).toBuffer('le', 16);

    return keccak256(Buffer.concat([_index, _account, _amount]));
  }

  public getHexRoot(): string {
    return this.tree.getHexRoot();
  }

  public getProof(index: BN, account: string, amount: BN): string[] {
    return this.tree.getHexProof(BalanceTree.toNode(index, account, amount));
  }
}
