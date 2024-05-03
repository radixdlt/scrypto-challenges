export type Openrpc = "1.2.6" | "1.2.5" | "1.2.4" | "1.2.3" | "1.2.2" | "1.2.1" | "1.2.0" | "1.1.12" | "1.1.11" | "1.1.10" | "1.1.9" | "1.1.8" | "1.1.7" | "1.1.6" | "1.1.5" | "1.1.4" | "1.1.3" | "1.1.2" | "1.1.1" | "1.1.0" | "1.0.0" | "1.0.0-rc1" | "1.0.0-rc0";
export type InfoObjectProperties = string;
export type InfoObjectDescription = string;
export type InfoObjectTermsOfService = string;
export type InfoObjectVersion = string;
export type ContactObjectName = string;
export type ContactObjectEmail = string;
export type ContactObjectUrl = string;
export type SpecificationExtension = any;
export interface ContactObject {
  name?: ContactObjectName;
  email?: ContactObjectEmail;
  url?: ContactObjectUrl;
  [regex: string]: SpecificationExtension | any;
}
export type LicenseObjectName = string;
export type LicenseObjectUrl = string;
export interface LicenseObject {
  name?: LicenseObjectName;
  url?: LicenseObjectUrl;
  [regex: string]: SpecificationExtension | any;
}
export interface InfoObject {
  title: InfoObjectProperties;
  description?: InfoObjectDescription;
  termsOfService?: InfoObjectTermsOfService;
  version: InfoObjectVersion;
  contact?: ContactObject;
  license?: LicenseObject;
  [regex: string]: SpecificationExtension | any;
}
export type ExternalDocumentationObjectDescription = string;
export type ExternalDocumentationObjectUrl = string;
/**
 *
 * information about external documentation
 *
 */
export interface ExternalDocumentationObject {
  description?: ExternalDocumentationObjectDescription;
  url: ExternalDocumentationObjectUrl;
  [regex: string]: SpecificationExtension | any;
}
export type ServerObjectUrl = string;
export type ServerObjectName = string;
export type ServerObjectDescription = string;
export type ServerObjectSummary = string;
export type ServerObjectVariableDefault = string;
export type ServerObjectVariableDescription = string;
export type ServerObjectVariableEnumItem = string;
export type ServerObjectVariableEnum = ServerObjectVariableEnumItem[];
export interface ServerObjectVariable {
  default: ServerObjectVariableDefault;
  description?: ServerObjectVariableDescription;
  enum?: ServerObjectVariableEnum;
  [k: string]: any;
}
export interface ServerObjectVariables { [key: string]: any; }
export interface ServerObject {
  url: ServerObjectUrl;
  name?: ServerObjectName;
  description?: ServerObjectDescription;
  summary?: ServerObjectSummary;
  variables?: ServerObjectVariables;
  [regex: string]: SpecificationExtension | any;
}
export type Servers = ServerObject[];
/**
 *
 * The cannonical name for the method. The name MUST be unique within the methods array.
 *
 */
export type MethodObjectName = string;
/**
 *
 * A verbose explanation of the method behavior. GitHub Flavored Markdown syntax MAY be used for rich text representation.
 *
 */
export type MethodObjectDescription = string;
/**
 *
 * A short summary of what the method does.
 *
 */
export type MethodObjectSummary = string;
export type TagObjectName = string;
export type TagObjectDescription = string;
export interface TagObject {
  name: TagObjectName;
  description?: TagObjectDescription;
  externalDocs?: ExternalDocumentationObject;
  [regex: string]: SpecificationExtension | any;
}
export type $Ref = string;
export interface ReferenceObject {
  $ref: $Ref;
}
export type TagOrReference = TagObject | ReferenceObject;
export type MethodObjectTags = TagOrReference[];
/**
 *
 * Format the server expects the params. Defaults to 'either'.
 *
 * @default either
 *
 */
export type MethodObjectParamStructure = "by-position" | "by-name" | "either";
export type ContentDescriptorObjectName = string;
export type ContentDescriptorObjectDescription = string;
export type ContentDescriptorObjectSummary = string;
export type $Id = string;
export type $Schema = string;
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
export type SimpleTypes = any;
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
export type ContentDescriptorObjectRequired = boolean;
export type ContentDescriptorObjectDeprecated = boolean;
export interface ContentDescriptorObject {
  name: ContentDescriptorObjectName;
  description?: ContentDescriptorObjectDescription;
  summary?: ContentDescriptorObjectSummary;
  schema: JSONSchema;
  required?: ContentDescriptorObjectRequired;
  deprecated?: ContentDescriptorObjectDeprecated;
  [regex: string]: SpecificationExtension | any;
}
export type ContentDescriptorOrReference = ContentDescriptorObject | ReferenceObject;
export type MethodObjectParams = ContentDescriptorOrReference[];
export type MethodObjectResult = ContentDescriptorObject | ReferenceObject;
/**
 *
 * A Number that indicates the error type that occurred. This MUST be an integer. The error codes from and including -32768 to -32000 are reserved for pre-defined errors. These pre-defined errors SHOULD be assumed to be returned from any JSON-RPC api.
 *
 */
export type ErrorObjectCode = number;
/**
 *
 * A String providing a short description of the error. The message SHOULD be limited to a concise single sentence.
 *
 */
export type ErrorObjectMessage = string;
/**
 *
 * A Primitive or Structured value that contains additional information about the error. This may be omitted. The value of this member is defined by the Server (e.g. detailed error information, nested errors etc.).
 *
 */
export type ErrorObjectData = any;
/**
 *
 * Defines an application level error.
 *
 */
export interface ErrorObject {
  code: ErrorObjectCode;
  message: ErrorObjectMessage;
  data?: ErrorObjectData;
}
export type ErrorOrReference = ErrorObject | ReferenceObject;
/**
 *
 * Defines an application level error.
 *
 */
export type MethodObjectErrors = ErrorOrReference[];
export type LinkObjectName = string;
export type LinkObjectSummary = string;
export type LinkObjectMethod = string;
export type LinkObjectDescription = string;
export type LinkObjectParams = any;
export interface LinkObjectServer {
  url: ServerObjectUrl;
  name?: ServerObjectName;
  description?: ServerObjectDescription;
  summary?: ServerObjectSummary;
  variables?: ServerObjectVariables;
  [regex: string]: SpecificationExtension | any;
}
export interface LinkObject {
  name?: LinkObjectName;
  summary?: LinkObjectSummary;
  method?: LinkObjectMethod;
  description?: LinkObjectDescription;
  params?: LinkObjectParams;
  server?: LinkObjectServer;
  [regex: string]: SpecificationExtension | any;
}
export type LinkOrReference = LinkObject | ReferenceObject;
export type MethodObjectLinks = LinkOrReference[];
export type ExamplePairingObjectName = string;
export type ExamplePairingObjectDescription = string;
export type ExampleObjectSummary = string;
export type ExampleObjectValue = any;
export type ExampleObjectDescription = string;
export type ExampleObjectName = string;
export interface ExampleObject {
  summary?: ExampleObjectSummary;
  value: ExampleObjectValue;
  description?: ExampleObjectDescription;
  name: ExampleObjectName;
  [regex: string]: SpecificationExtension | any;
}
export type ExampleOrReference = ExampleObject | ReferenceObject;
export type ExamplePairingObjectParams = ExampleOrReference[];
export type ExamplePairingObjectResult = ExampleObject | ReferenceObject;
export interface ExamplePairingObject {
  name: ExamplePairingObjectName;
  description?: ExamplePairingObjectDescription;
  params: ExamplePairingObjectParams;
  result: ExamplePairingObjectResult;
  [k: string]: any;
}
export type ExamplePairingOrReference = ExamplePairingObject | ReferenceObject;
export type MethodObjectExamples = ExamplePairingOrReference[];
export type MethodObjectDeprecated = boolean;
export interface MethodObject {
  name: MethodObjectName;
  description?: MethodObjectDescription;
  summary?: MethodObjectSummary;
  servers?: Servers;
  tags?: MethodObjectTags;
  paramStructure?: MethodObjectParamStructure;
  params: MethodObjectParams;
  result: MethodObjectResult;
  errors?: MethodObjectErrors;
  links?: MethodObjectLinks;
  examples?: MethodObjectExamples;
  deprecated?: MethodObjectDeprecated;
  externalDocs?: ExternalDocumentationObject;
  [regex: string]: SpecificationExtension | any;
}
export type MethodOrReference = MethodObject | ReferenceObject;
export type Methods = MethodOrReference[];
export interface SchemaComponents { [key: string]: any; }
export interface LinkComponents { [key: string]: any; }
export interface ErrorComponents { [key: string]: any; }
export interface ExampleComponents { [key: string]: any; }
export interface ExamplePairingComponents { [key: string]: any; }
export interface ContentDescriptorComponents { [key: string]: any; }
export interface TagComponents { [key: string]: any; }
export interface Components {
  schemas?: SchemaComponents;
  links?: LinkComponents;
  errors?: ErrorComponents;
  examples?: ExampleComponents;
  examplePairings?: ExamplePairingComponents;
  contentDescriptors?: ContentDescriptorComponents;
  tags?: TagComponents;
  [k: string]: any;
}
export interface OpenrpcDocument {
  openrpc: Openrpc;
  info: InfoObject;
  externalDocs?: ExternalDocumentationObject;
  servers?: Servers;
  methods: Methods;
  components?: Components;
  [regex: string]: SpecificationExtension | any;
}