/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      extend: {},
    },
    plugins: [require("daisyui")],
    darkMode: "class",
    daisyui: {
        themes: ["light", "night", "dark"],
    },
}