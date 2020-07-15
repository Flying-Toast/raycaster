export function decodePacket(packet) {
	let messages = [];
	let lines = packet.split("\n");
	while (lines.length != 0) {
		messages.push(nextMessage(lines));
	}
	return messages;
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

	static parse(lines) {
		let entityID = Number(lines.shift());
		return new YourIDPayload(entityID);
	}
}

function nextMessage(lines) {
	const payloadKey = lines.shift();

	switch (payloadKey) {
		case "u":
			return YourIDPayload.parse(lines);
		default:
			throw new Error(`unknown payload key "${payloadKey}"`);
	}
}
