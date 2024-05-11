build: 
	(cd game-frontend && npm run build)
	(cd game-lib && cargo build)
	(cd game-server && cargo build)
	rm -rf  game-server/static/
	mv game-frontend/dist game-server/static 
	gsed -i 's/crossorigin//g' game-server/static/index.html
	gsed -i 's/\/assets/\/static\/assets/g' game-server/static/index.html
	gsed -i 's/\/icon.svg/\/static\/icon.svg/g' game-server/static/index.html

run: build
	(cd game-server && cargo run)

build-docker: build
	docker build -t game-server  -f game-server/Dockerfile .

run-docker: 
	docker run -p 443:443 game-server
