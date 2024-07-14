export const shortenAddress = (address?: string) => {
  if (!address) {
    console.warn('Address is undefined');
    return '';
  }

  return `${address.slice(0, 4)}...${address.slice(address.length - 6, address.length)}`;
};
