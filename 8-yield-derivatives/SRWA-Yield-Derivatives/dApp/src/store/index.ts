import { create } from 'zustand';

import { AssetsItemModel } from '../app/shared/models/index.model';

type BearStoreState = {
  accountAddress: string;
  accountAssets: AssetsItemModel[];
  yieldAssets: AssetsItemModel[];
  componentAssets: AssetsItemModel[];
  userBadge: string;
};

export const useBearStore = create<BearStoreState>(() => ({
  accountAddress: '',
  accountAssets: [],
  yieldAssets: [],
  componentAssets: [],
  userBadge: '',
}));
