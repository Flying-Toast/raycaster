import { decodePacket } from "./protocol.js";


class Responder {
	constructor(ws) {
		this._ws = ws;
	}

	send(message) {
		this._ws.send(message);
	}
}

export class Network {
	constructor() {
		this.messageBuffer = [];
	}

	connect() {
		this.ws = new WebSocket("ws://localhost:8000");
		this.ws.addEventListener("message", m => this.handlePacket(m.data));
	}

	handlePacket(packet) {
		for (const message of decodePacket(packet)) {
			this.messageBuffer.push(message);
		}
	}

	drainMessages() {
		let drained = this.messageBuffer;
		this.messageBuffer = [];
		return drained;
	}

	makeResponder() {
		return new Responder(this.ws);
	}
}
