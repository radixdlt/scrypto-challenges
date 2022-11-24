import {parseAddress} from "./utils";

export class AppState {
    accountAddress?: string;
    radSenseComponent?: string;
    rsa?: RadSenseAddresses;
    arbitrationDaoAddresses?: DaoSystemAddresses;
    adBrokerId?: string;
    advertiserId?: string;
    adSlotProviderId?: string;
    adIds: Array<string> = [];
    adSlotIds: Array<string> = [];
}

export class RadSenseAddresses {
    readonly ad_slot_resource: string;
    readonly ad_resource: string;
    readonly advertiser_resource: string;
    readonly ad_slot_provider_resource: string;
    readonly ad_broker_resource: string;
    readonly access_badge_resource: string;
    // readonly kyc_resources: Set<string>; // Not required atm

    constructor(sborJson: { type: string, fields: Array<{ type: string, value: string }> }) {
        this.ad_slot_resource = parseAddress(sborJson.fields[0]);
        this.ad_resource = parseAddress(sborJson.fields[1]);
        this.advertiser_resource = parseAddress(sborJson.fields[2]);
        this.ad_slot_provider_resource = parseAddress(sborJson.fields[3]);
        this.ad_broker_resource = parseAddress(sborJson.fields[4]);
        this.access_badge_resource = parseAddress(sborJson.fields[5]);
    }
}

export class DaoSystemAddresses {
    readonly dao_system_component: string;
    readonly membership_system_component: string;
    readonly code_execution_system_component: string;
    readonly voting_system_component: string;
    readonly membership_resource: string;
    readonly code_execution_resource: string;
    readonly vote_resource: string;
    readonly vote_receipt_resource: string;
    readonly dao_system_admin_badge_resource: string;

    constructor(sborJson: { type: string, fields: Array<{ type: string, value: string }> }) {
        this.dao_system_component = parseAddress(sborJson.fields[0]);
        this.membership_system_component = parseAddress(sborJson.fields[1]);
        this.code_execution_system_component = parseAddress(sborJson.fields[2]);
        this.voting_system_component = parseAddress(sborJson.fields[3]);
        this.membership_resource = parseAddress(sborJson.fields[4]);
        this.code_execution_resource = parseAddress(sborJson.fields[5]);
        this.vote_resource = parseAddress(sborJson.fields[6]);
        this.vote_receipt_resource = parseAddress(sborJson.fields[7]);
        this.dao_system_admin_badge_resource = parseAddress(sborJson.fields[8]);
    }
}

export type RadSenseEvent = {
    id: string;
    type: string;
    timestamp: number;
    adPlacementId: string;
    adNonFungibleId: string;
    adSlotNonFungibleId: string;
    adBrokerNonFungibleId: string;
}