import * as PLD from "./protocol.js";

export class Entity {
	constructor(id, location) {
		this.id = id;
		this.location = location;
	}
}

export class Game {
	constructor(responder) {
		this.responder = responder;
		this.myEntityID = null;
		this.entities = new Map();
	}

	handleMessage(message) {
		switch (message.type) {
			case "YourID": {
				this.myEntityID = message.entityID;
				//TEMP
				this.responder.send(new PLD.ClientHelloPayload("👋Hello from the client", 1234));
				break;
			}
			case "RemoveEntity": {
				this.entities.delete(message.entityID);
				break;
			}
			case "NewEntity": {
				this.entities.set(message.entity.id, message.entity);
				break;
			}
			default:
				console.error("Error handling message: ", message);
				throw new Error("Error handling message");
		}
	}
}
