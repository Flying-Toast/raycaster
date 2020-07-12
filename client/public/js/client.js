import { Network } from "./network.js";
import { Game } from "./game.js";
import { Renderer } from "./render.js";


export class Client {
	constructor() {
		this.net = new Network();
		this.game = null;
		this.renderer = new Renderer();
	}

	start() {
		this.net.connect();
		this.game = new Game(this.net.makeResponder());
		this.mainLoop();
	}

	mainLoop() {
		for (const message of this.net.drainMessages()) {
			this.game.handleMessage(message);
		}

		this.renderer.render(this.game);

		requestAnimationFrame(() => this.mainLoop());
	}
}
