import type {Readable, Writable} from "svelte/store";
import {writable} from "svelte/store";

export type MessageType = "Success" | "Error"

class MessageManager {
    public readonly message: Readable<string | null> = writable(null)
    public readonly messageType: Readable<MessageType | null> = writable(null)

    showMessage(text: string, type: MessageType) {
        // Reset to null to force an update if the text doesn't change between two error messages
        (this.message as Writable<string | null>).set(null);
        (this.message as Writable<string | null>).set(text);
        (this.messageType as Writable<MessageType | null>).set(type)
    }
}

export default new MessageManager()