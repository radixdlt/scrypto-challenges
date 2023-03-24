// Code copied from https://github.com/0xOmarA/RET-TS-POC and modified
// Original work by 0xOmarA (https://github.com/0xOmarA)

export default class RadixEngineToolkit {
    /**
     * The WASM instance of the Radix Engine Toolkit.
     * @private
     */
    private wasmInstance: WebAssembly.Instance;

    /**
     * The foreign function interface and objects exported by the WebAssembly instance of the RadixEngineToolkit
     * @private
     */
    private internalFFI: RadixEngineToolkitFFI;

    /**
     * Creates new RadixEngineToolkit instance from an instance of the WASM module.
     * @param wasmInstance An instance of the Radix Engine Toolkit WASM module
     */
    constructor(wasmInstance: WebAssembly.Instance) {
        this.wasmInstance = wasmInstance;
        this.internalFFI = wasmInstance.exports as unknown as RadixEngineToolkitFFI;
    }

    public sborDecode(sborHex: string, networkId: number): string | undefined {
        let request: object = {
            "encoded_value": sborHex,
            "network_id": networkId.toString()
        };
        let response: any = this.callWasmFunction(request, this.internalFFI.sbor_decode);
        return response?.["value"]
    }

    /**
     * A high-level method for calling functions from the `RadixEngineToolkitFFI` through a simple interface.
     *
     * The main purpose of this method is to provide a higher-level interface for calling into the `RadixEngineToolkit`,
     * as such, this method performs all required memory allocation, deallocation, object serialization, deserialization
     * encoding, and decoding required for any call into the RadixEngineToolkit
     * @param request An object containing the request payload
     * @param wasmFunction The function to call of the `RadixEngineToolkitFFI`
     * @return A generic object of type `O` of the response to the request
     * @private
     */
    private callWasmFunction<I, O>(
        request: I,
        wasmFunction: (pointer: number) => number
    ): O {
        // Write the request object to memory and get a pointer to where it was written
        let requestPointer: number = this.writeObjectToMemory(request as unknown as object);

        // Call the WASM function with the request pointer
        let responsePointer: number = wasmFunction(requestPointer);

        // Read and deserialize the response
        let response: O = this.readObjectFromMemory(responsePointer);

        // Deallocate the request and response pointers
        this.deallocateMemory(requestPointer);
        this.deallocateMemory(responsePointer);

        // Return the object back to the caller
        return response;
    }

    /**
     * Allocates memory of a certain capacity on the WebAssembly instance's linear memory through the
     * `RadixEngineToolkit`'s internal memory allocator
     * @param capacity The capacity of the memory to allocate
     * @return A memory pointer of the allocated memory
     * @private
     */
    private allocateMemory(capacity: number): number {
        return this.internalFFI.toolkit_alloc(capacity);
    }

    /**
     * Deallocates memory beginning from the provided memory pointer and ending at the first null-terminator found
     * @param pointer A memory pointer to the starting location of the memory to deallocate
     * @private
     */
    private deallocateMemory(pointer: number) {
        this.internalFFI.toolkit_free_c_string(pointer);
    }

    /**
     * Serializes an object to a JSON string
     * @param object The object to serialize
     * @return A string of the serialized representation
     * @private
     */
    private serializeObject(object: Object): string {
        return JSON.stringify(object)
    }

    /**
     * Deserializes a JSON string to an object of the generic type `T`.
     * @param string The JSON string to deserialize.
     * @return A generic object of type T deserialized from the JSON string.
     * @private
     */
    private deserializeString<T>(string: string): T {
        return JSON.parse(string) as T;
    }

    /**
     * A method to write strings to memory in the way expected by the Radix Engine Toolkit.
     *
     * This method first UTF-8 encodes the passed string and adds a null-terminator to it. It then allocates enough
     * memory for the encoded string and writes it to memory. Finally, this method returns the pointer back to the
     * caller to use.
     *
     * Note: Since the pointer is returned to the caller, it is now the caller's burden to deallocate this memory when
     * it is no longer needed.
     *
     * @param str A string to write to memory
     * @return A pointer to the memory location containing the null-terminated UTF-8 encoded string
     * @private
     */
    private writeStringToMemory(str: string): number {
        // UTF-8 encode the string and add the null terminator to it.
        let nullTerminatedUtf8EncodedString: Uint8Array = new Uint8Array([...new TextEncoder().encode(str), 0]);

        // Allocate memory for the string
        let memoryPointer: number = this.allocateMemory(nullTerminatedUtf8EncodedString.length);

        // Write the string to the instance's linear memory
        const view: Uint8Array = new Uint8Array(this.internalFFI.memory.buffer, memoryPointer);
        view.set(nullTerminatedUtf8EncodedString);

        // return the memory pointer back to the caller
        return memoryPointer;
    }

    /**
     * This method reads a UTF-8 null-terminated string the instance's linear memory and returns it as a JS string.
     * @param pointer A pointer to the memory location containing the string
     * @return A JS string of the read and decoded string
     * @private
     */
    private readStringFromMemory(pointer: number): string {
        // Determine the length of the string based on the first null terminator
        const view: Uint8Array = new Uint8Array(this.internalFFI.memory.buffer, pointer);
        const length: number = view.findIndex((byte) => byte === 0);

        // Read the UTF-8 encoded string from memory
        let nullTerminatedUtf8EncodedString: Uint8Array = new Uint8Array(this.internalFFI.memory.buffer, pointer, length);

        // Decode the string and return it back to the caller
        return new TextDecoder().decode(nullTerminatedUtf8EncodedString);
    }

    /**
     * Writes an object to memory by serializing it to JSON and UTF-8 encoding the serialized string.
     * @param obj The object to write to the instance's linear memory.
     * @return A pointer to the location of the object in memory
     * @private
     */
    private writeObjectToMemory(obj: object): number {
        // Serialize the object to json
        let serializedObject: string = this.serializeObject(obj);

        // Write the string to memory and return the pointer
        return this.writeStringToMemory(serializedObject);
    }

    /**
     * Reads a UTF-8 encoded JSON string from memory and deserializes it as `T`.
     * @param pointer A memory pointer to the location of the linear memory where the object lives.
     * @return An object of type `T` of the deserialized object
     * @private
     */
    private readObjectFromMemory<T>(pointer: number): T {
        // Read the UTF-8 encoded null-terminated string from memory
        let serializedObject: string = this.readStringFromMemory(pointer);

        // Deserialize and return to the caller
        return this.deserializeString(serializedObject);
    }
}

interface RadixEngineToolkitFFI {
    /**
     * The Radix Engine Toolkit WASM exports its own memory and does not require any memory imports. This is the memory
     * exported by the WebAssembly instance.
     */
    memory: WebAssembly.Memory

    decompile_unknown_transaction_intent(requestStringPointer: number): number;

    sbor_decode(requestStringPointer: number): number;

    /**
     * A foreign function interface for the toolkit function responsible for all allocation of memory used in the
     * toolkit
     * @param capacity The capacity of the memory to allocate.
     * @return A memory pointer pointing to the start of the allocated memory
     */
    toolkit_alloc(capacity: number): number;

    /**
     * A foreign function interface for the toolkit function responsible for the deallocation of memory.
     *
     * It should be noted that this function operates with two main assumptions:
     * 1. That the memory that will be freed has been allocated with the same allocator.
     * 2. That the memory contains a null-terminated c-string.
     *
     * Therefore, this function does not require any additional information as to the size of the memory to deallocate,
     * this will be determined based on the first null-terminator encountered.
     *
     * @param pointer A pointer to the start of the memory to free.
     */
    toolkit_free_c_string(pointer: number): void;
}
