export class Entity {
	constructor(location) {
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
				break;
			}
			case "RemoveEntity": {
				this.entities.delete(message.entityID);
				break;
			}
			case "NewEntity": {
				this.entities.set(message.entityID, message.entity);
				break;
			}
			default:
				console.error("Error handling message: ", message);
				throw new Error("Error handling message");
		}
	}
}
