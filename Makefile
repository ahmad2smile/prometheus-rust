start-api:
	systemfd --no-pid -s http::3000 -- cargo watch -x run
