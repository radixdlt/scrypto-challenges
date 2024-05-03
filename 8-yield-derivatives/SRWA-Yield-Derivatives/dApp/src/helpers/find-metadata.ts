export const getStringMetadata = (key: string, object: { metadata: any[] }) => {
  const metadata = object.metadata.find((m) => m.key === key);
  return metadata ? metadata.value.typed.value : undefined;
};
