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
				this.entities.remove(message.entityID);
				break;
			}
			default:
				console.error("Error handling message: ", message);
				throw new Error("Error handling message");
		}
	}
}
