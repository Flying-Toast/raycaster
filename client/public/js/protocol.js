class Pieces {
	constructor(string) {
		this.lines = string.split("\n");
	}

	getString() {
		if (this.empty()) {
			throw new Error("Empty pieces");
		}
		return this.lines.shift();
	}

	getInt() {
		let string = this.getString();
		let parsed = parseInt(string, 10);
		if (Number.isNaN(parsed)) {
			throw new Error(`Error parsing int from "${string}"`);
		}
		return parsed;
	}

	getFloat() {
		return (
			new Float32Array(
				Uint32Array.of(this.getInt()).buffer
			)
		)[0];
	}

	empty() {
		return this.lines.length == 0;
	}
}

export function decodePacket(packet) {
	let payloads = [];
	let pieces = new Pieces(packet);
	while (!pieces.empty()) {
		payloads.push(nextMessage(pieces));
	}
	return payloads;
}

class IncomingPayload {
	constructor(type) {
		this.type = type;
	}
}

class YourIDPayload extends IncomingPayload {
	constructor(entityID) {
		super("YourID");
		this.entityID = entityID;
	}

	static parse(pieces) {
		let entityID = pieces.getInt();
		return new YourIDPayload(entityID);
	}
}

class RemoveEntityPayload extends IncomingPayload {
	constructor(entityID) {
		super("RemoveEntity");
		this.entityID = entityID;
	}

	static parse(pieces) {
		let id = pieces.getInt();
		return new RemoveEntityPayload(id);
	}
}

function nextMessage(pieces) {
	const payloadKey = pieces.getString();

	switch (payloadKey) {
		case "u":
			return YourIDPayload.parse(pieces);
		case "r":
			return RemoveEntityPayload.parse(pieces);
		default:
			throw new Error(`unknown payload key "${payloadKey}"`);
	}
}
