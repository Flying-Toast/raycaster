let game = null;
let renderer = null;
let lastTime = null;

export function runGameLoop(game_, renderer_) {
	game = game_;
	renderer = renderer_;
	requestAnimationFrame(gameLoop);
}

function gameLoop(currentTime) {
	if (lastTime == null) {
		lastTime = currentTime;
		requestAnimationFrame(gameLoop);
		return;
	}

	const dt = currentTime - lastTime;

	game.update(dt);
	renderer.render(game);

	lastTime = currentTime;
	requestAnimationFrame(gameLoop);
}
