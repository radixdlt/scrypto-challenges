export type $Id = string;
export type $Schema = string;
export type $Ref = string;
export type $Comment = string;
export type Title = string;
export type Description = string;
type AlwaysTrue = any;
export type ReadOnly = boolean;
export type Examples = AlwaysTrue[];
export type MultipleOf = number;
export type Maximum = number;
export type ExclusiveMaximum = number;
export type Minimum = number;
export type ExclusiveMinimum = number;
export type NonNegativeInteger = number;
export type NonNegativeIntegerDefaultZero = number;
export type Pattern = string;
export type SchemaArray = JSONSchema[];
/**
 *
 * @default true
 *
 */
export type Items = JSONSchema | SchemaArray;
export type UniqueItems = boolean;
export type StringDoaGddGA = string;
/**
 *
 * @default []
 *
 */
export type StringArray = StringDoaGddGA[];
/**
 *
 * @default {}
 *
 */
export interface Definitions { [key: string]: any; }
/**
 *
 * @default {}
 *
 */
export interface Properties { [key: string]: any; }
/**
 *
 * @default {}
 *
 */
export interface PatternProperties { [key: string]: any; }
export type DependenciesSet = JSONSchema | StringArray;
export interface Dependencies { [key: string]: any; }
export type Enum = AlwaysTrue[];
export type SimpleTypes = "array" | "boolean" | "integer" | "null" | "number" | "object" | "string";
export type ArrayOfSimpleTypes = SimpleTypes[];
export type Type = SimpleTypes | ArrayOfSimpleTypes;
export type Format = string;
export type ContentMediaType = string;
export type ContentEncoding = string;
export interface JSONSchemaObject {
  $id?: $Id;
  $schema?: $Schema;
  $ref?: $Ref;
  $comment?: $Comment;
  title?: Title;
  description?: Description;
  default?: AlwaysTrue;
  readOnly?: ReadOnly;
  examples?: Examples;
  multipleOf?: MultipleOf;
  maximum?: Maximum;
  exclusiveMaximum?: ExclusiveMaximum;
  minimum?: Minimum;
  exclusiveMinimum?: ExclusiveMinimum;
  maxLength?: NonNegativeInteger;
  minLength?: NonNegativeIntegerDefaultZero;
  pattern?: Pattern;
  additionalItems?: JSONSchema;
  items?: Items;
  maxItems?: NonNegativeInteger;
  minItems?: NonNegativeIntegerDefaultZero;
  uniqueItems?: UniqueItems;
  contains?: JSONSchema;
  maxProperties?: NonNegativeInteger;
  minProperties?: NonNegativeIntegerDefaultZero;
  required?: StringArray;
  additionalProperties?: JSONSchema;
  definitions?: Definitions;
  properties?: Properties;
  patternProperties?: PatternProperties;
  dependencies?: Dependencies;
  propertyNames?: JSONSchema;
  const?: AlwaysTrue;
  enum?: Enum;
  type?: Type;
  format?: Format;
  contentMediaType?: ContentMediaType;
  contentEncoding?: ContentEncoding;
  if?: JSONSchema;
  then?: JSONSchema;
  else?: JSONSchema;
  allOf?: SchemaArray;
  anyOf?: SchemaArray;
  oneOf?: SchemaArray;
  not?: JSONSchema;
  [k: string]: any;
}
/**
 *
 * Always valid if true. Never valid if false. Is constant.
 *
 */
export type JSONSchemaBoolean = boolean;
/**
 *
 * @default {}
 *
 */
export type JSONSchema = JSONSchemaObject | JSONSchemaBoolean;