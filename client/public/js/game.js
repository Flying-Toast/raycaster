export class Game {
	constructor(responder) {
		this.responder = responder;
		this.myEntityID = null;
	}

	handleMessage(message) {
		switch (message.type) {
			case "YourID": {
				this.myEntityID = message.entityID;
				break;
			}
		}
	}
}
