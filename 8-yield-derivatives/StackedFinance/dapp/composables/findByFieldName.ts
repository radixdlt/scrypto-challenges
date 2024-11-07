//This will help search the gateway response fields for particular fields
//Handles any that have a value like Reference, Own, etc
export const useFindByFieldName = (dataArray, fieldName) => {
  // Method to find the object by field_name
  const findByFieldName = dataArray.find(
    (field) => field.field_name === fieldName
  )?.value;
  // Return the reactive reference and the method
  return { findByFieldName };
};

//This is to handle gateway responses that contain fields instead of values
//For example a tuple
export const useFindTuple = (dataArray, fieldName) => {
  // Method to find the object by field_name
  const findByFieldName = dataArray.find(
    (field) => field.field_name === fieldName
  )?.fields;
  // Return the reactive reference and the method
  return { findByFieldName };
};