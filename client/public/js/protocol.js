import { Vector } from "./vector.js";
import { Entity } from "./game.js";


class Pieces {
	constructor(view) {
		this.view = view;
		this.offset = 0;
		this.decoder = new TextDecoder();
	}

	getString() {
		const len = this.getUint32();
		this.assertAtLeastNBytes(len);
		let bytes = new Uint8Array(len);
		for (let i = 0; i < len; i++) {
			bytes[i] = this.getUint8();
		}
		return this.decoder.decode(bytes.buffer);
	}

	getUint8() {
		this.assertAtLeastNBytes(8/8);
		const int = this.view.getUint8(this.offset);
		this.offset += 8/8;
		return int;
	}

	getUint32() {
		this.assertAtLeastNBytes(32/8);
		const int = this.view.getUint32(this.offset);
		this.offset += 32/8;
		return int;
	}

	getUint16() {
		this.assertAtLeastNBytes(16/8);
		const int = this.view.getUint16(this.offset);
		this.offset += 16/8;
		return int;
	}

	getFloat32() {
		this.assertAtLeastNBytes(32/8);
		const float = this.view.getFloat32(this.offset);
		this.offset += 32/8;
		return float;
	}

	getVector() {
		let x = this.getFloat32();
		let y = this.getFloat32();
		return new Vector(x, y);
	}

	assertAtLeastNBytes(n) {
		const remaining = this.view.byteLength - this.offset;
		if (remaining < n) {
			throw new Error("Not enough bytes left");
		}
	}

	empty() {
		return this.offset >= this.view.byteLength;
	}
}

export function decodePacket(packet) {
	let payloads = [];
	let pieces = new Pieces(new DataView(packet));
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
		let entityID = pieces.getUint32();
		return new YourIDPayload(entityID);
	}
}

class RemoveEntityPayload extends IncomingPayload {
	constructor(entityID) {
		super("RemoveEntity");
		this.entityID = entityID;
	}

	static parse(pieces) {
		let id = pieces.getUint32();
		return new RemoveEntityPayload(id);
	}
}

class NewEntityPayload extends IncomingPayload {
	constructor(entity) {
		super("NewEntity");
		this.entity = entity;
	}

	static parse(pieces) {
		let id = pieces.getUint32();
		let location = pieces.getVector();
		return new NewEntityPayload(new Entity(id, location));
	}
}

function nextMessage(pieces) {
	const payloadKey = pieces.getUint16();

	switch (payloadKey) {
		case 1:
			return YourIDPayload.parse(pieces);
		case 2:
			return RemoveEntityPayload.parse(pieces);
		case 3:
			return NewEntityPayload.parse(pieces);
		default:
			throw new Error(`unknown payload key "${payloadKey}"`);
	}
}
