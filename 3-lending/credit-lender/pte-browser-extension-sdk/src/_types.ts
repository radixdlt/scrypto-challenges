import { Receipt } from 'pte-sdk';

/**
 * Represents the type of an action.
 */
export enum ActionType {
  GetAccountAddress = "getAccountAddress",
  GetAccountAddressSuccess = "getAccountAddressSuccess",
  GetAccountAddressFailure = "getAccountAddressFailure",

  SignTransaction = "signTransaction",
  SignTransactionSuccess = "signTransactionSuccess",
  SignTransactionFailure = "signTransactionFailure",
}

/**
 * Represents an action.
 */
export type Action<T extends ActionType, P> = {
  type: T;
  payload: P;
  id: string;
};

export type GetAccountAddress = Action<ActionType.GetAccountAddress, string>;
export type GetAccountAddressSuccess = Action<ActionType.GetAccountAddressSuccess, string>;
export type GetAccountAddressFailure = Action<ActionType.GetAccountAddressFailure, string>;

export type SignTransaction = Action<ActionType.SignTransaction, string>;
export type SignTransactionSuccess = Action<ActionType.SignTransactionSuccess, Receipt>;
export type SignTransactionFailure = Action<ActionType.SignTransactionFailure, string>;

export type ActionTypes =
  GetAccountAddress
  | GetAccountAddressSuccess
  | GetAccountAddressFailure
  | SignTransaction
  | SignTransactionSuccess
  | SignTransactionFailure

export enum MessageTarget {
  Extension,
  Dapp,
}

export type Message<Action = ActionTypes> = {
  action: Action;
  target: MessageTarget;
};

export type MessageSenderData = {
  tabId: number;
  url: string;
  createdAt: number;
};

export type MessageStoreItem<Action = ActionTypes> = Message<Action> &
  MessageSenderData;
