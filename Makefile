t:
	npx tailwindcss -i ./input.css --config ./tailwind.config.js -o ./style/output.css
tw:
	npx tailwindcss -i ./input.css --config ./tailwind.config.js -o ./style/output.css --watch
l:
	cargo leptos build
lw:
	cargo leptos watch
tlw:
	make t
	make tw & make lw