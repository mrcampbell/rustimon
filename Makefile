MAKEFLAGS += -j2

tailwind:
	cd server/public npx tailwindcss -i ./styles/index.css -o ./styles/tailwind.css --watch

serve:
	cd server && dioxus serve

dev:
	$(MAKE) -j tailwind serve