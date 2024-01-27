build: 
	(cd game-frontend && npm run build)
	(cd game-lib && cargo build)
	(cd game-server && cargo build)
	rm -rf  game-server/static/
	mv game-frontend/dist game-server/static 