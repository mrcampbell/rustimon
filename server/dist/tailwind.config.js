/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["/Users/mikecampbell/Learn/dioxis-demo/hello/**/*.{html,js}"],
  theme: {
    extend: {},
  },
  plugins: [require('daisyui')],
}
