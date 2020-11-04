verify:
	go mod tidy
	go mod verify

test: verify
	go test -count=1 ./...
