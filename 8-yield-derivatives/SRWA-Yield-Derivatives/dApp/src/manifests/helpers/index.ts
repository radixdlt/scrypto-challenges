import { toast } from 'react-toastify';

type ErrorType = {
  interactionId: string;
  error: string;
};

type ResultType = {
  error?: ErrorType;
};

export const handleErrorManifest = (result: ResultType): void => {
  if (result.error) {
    toast.error(result.error.error);
  }
};
