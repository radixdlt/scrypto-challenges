import { StateEntityNonFungiblesPageResponse } from '@radixdlt/babylon-gateway-api-sdk';
import {
  FungibleResourcesCollectionItem,
  StateEntityDetailsResponse,
  StateEntityFungiblesPageResponse,
} from '@radixdlt/babylon-gateway-api-sdk';
import { StateEntityDetailsResponseItem } from '@radixdlt/babylon-gateway-api-sdk/dist/generated/models/StateEntityDetailsResponseItem';

import { AssetsItemModel } from '../../app/shared/models/index.model';
import { COMPONENT_ADDRESS, stateApi } from '../../helpers';

export const findUserBadgeAddress = async (data: StateEntityNonFungiblesPageResponse) => {
  for (const e of data.items) {
    const metadata = await stateApi.entityMetadataPage({
      stateEntityMetadataPageRequest: {
        address: e.resource_address,
      },
    });

    for (const item of metadata.items) {
      // @ts-expect-error: Property 'value' does not exist on type 'MetadataTypedValue'.
      if (item.key === 'component' && item.value?.typed?.value === COMPONENT_ADDRESS) {
        return metadata.address;
      }
    }
  }
};

export const setInitialAssets = (
  entityData: StateEntityDetailsResponse,
  fungiblesData: StateEntityFungiblesPageResponse,
) => {
  return entityData.items.flatMap((entity: StateEntityDetailsResponseItem) => {
    if (entity.metadata && entity.metadata.items) {
      return entity.metadata.items
        .filter((item: StateEntityDetailsResponseItem) => item.key === 'symbol')
        .map((item: StateEntityDetailsResponseItem) => {
          const fungibleItem = fungiblesData.items.find((fungible: FungibleResourcesCollectionItem) => {
            return fungible.resource_address === entity.address;
          }) as FungibleResourcesCollectionItem & { amount: number };

          return {
            as_string: item.value.typed.value,
            amount: fungibleItem ? Number(fungibleItem?.amount) : null,
            address: entity.address,
          };
        });
    }
    return [];
  });
};

export const updateAssetsFields = (assets: AssetsItemModel[], fields: any[], fieldName: string): AssetsItemModel[] =>
  assets.map((asset) => {
    const field = fields.find((field) => field.field_name === fieldName);
    const matchingItem = field?.entries.find((entry: any) => entry.key.value === asset.address);
    const updatedValue = matchingItem ? parseFloat(matchingItem.value.value) : 0;

    return {
      ...asset,
      [fieldName]: updatedValue,
    };
  });
