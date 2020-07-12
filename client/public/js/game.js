export class Game {
	constructor(responder) {
		this.responder = responder;
		this.myID = null;
	}

	handleMessage(message) {
		switch (message.type) {
			case "YourID": {
				this.myID = message.clientID;
				break;
			}
		}
	}
}
