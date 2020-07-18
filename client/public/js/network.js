import { decodePacket } from "./protocol.js";


class Responder {
	constructor(ws) {
		this._ws = ws;
		this.queue = [];
	}

	send(message) {
		this.queue.push(message);
	}

	flush() {
		if (this.queue.length == 0) {
			return;
		}
		let packet = this.queue.join("\n");
		this.queue = [];
		this._ws.send(packet);
	}
}

export class Network {
	constructor() {
		this.messageBuffer = [];
		this.responder = null;
	}

	connect() {
		this.ws = new WebSocket("ws://localhost:8000");
		this.ws.addEventListener("message", m => this.handlePacket(m.data));
		this.responder = new Responder(this.ws);
	}

	handlePacket(packet) {
		for (const message of decodePacket(packet)) {
			console.log("Got message: ", message);
			this.messageBuffer.push(message);
		}
	}

	drainMessages() {
		let drained = this.messageBuffer;
		this.messageBuffer = [];
		return drained;
	}

	getResponder() {
		return this.responder;
	}

	flush() {
		this.responder.flush();
	}
}
