import { cfg } from "./config.js";

export class Renderer {
	constructor(canvas) {
		this.width = cfg.resolution.width;
		this.height = cfg.resolution.height;
		canvas.width = this.width;
		canvas.height = this.height;
		this.canvas = canvas;
		this.ctx = canvas.getContext("2d");
	}

	render(game) {
		this.ctx.clearRect(0, 0, this.width, this.height);
		for (const row of game.map.tiles) {
			for (const t of row) {
				this.ctx.fillStyle = cfg.colors[t.type];
				this.ctx.fillRect(t.location.x * cfg.tileSize, t.location.y * cfg.tileSize, cfg.tileSize, cfg.tileSize);
			}
		}
	}
}
