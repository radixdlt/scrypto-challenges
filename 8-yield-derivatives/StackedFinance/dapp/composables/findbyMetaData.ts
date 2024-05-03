//Thsis is a way to parse metadata from entitiy details
export const useFindbyMetaData = (metadata, key) => {
    // Method to find the object by field_name
    const findByFieldName = metadata.items.find(
      (items) => items.key === key
    )?.value;
    // Return the reactive reference and the method
    return findByFieldName ;
  };