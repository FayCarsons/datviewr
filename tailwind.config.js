/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      fontFamily: {
        square: ["square", "Square"],
      },
    },
  },
  plugins: [],
};
