import { Account } from '@radixdlt/wallet-sdk';
import { BehaviorSubject } from 'rxjs';

import { addEntities } from '../entity/state';

const accounts = new BehaviorSubject<Account[]>([]);

export const setAccounts = (input: Account[]) => {
  accounts.next(input);
  addEntities(input.map((item) => ({ address: item.address, type: 'account' })));
};
