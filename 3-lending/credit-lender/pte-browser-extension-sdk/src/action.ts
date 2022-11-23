import { ActionType, ActionTypes, MessageStoreItem } from "./_types";

export const sendAction = ({ type, payload }: Omit<ActionTypes, "id">) => {
  const event = new CustomEvent("radix#chromeExtension#send", {
    detail: { type, payload },
  });

  window.dispatchEvent(event);
};

export const waitForAction = async <SuccessType extends ActionTypes>(
  successType: ActionType,
  errorTypes?: ActionType[]
): Promise<SuccessType> =>
  new Promise((resolve, reject) => {
    window.addEventListener(
      "radix#chromeExtension#receive",
      (event) => {
        const { action } = (event as CustomEvent<MessageStoreItem<ActionTypes>>)
          .detail;

        if (action.type === successType) resolve(action as SuccessType);
        else if (errorTypes?.includes(action.type)) reject(action);
      },
      {
        once: true,
      }
    );
  });