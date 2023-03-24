export type ApiPolicy = {
  name: string;
  description: string;
  price: string;
  nbTokenCovered: number;
  rollOverDate: Date;
};

export type ApiPolicies = ApiPolicy[];
