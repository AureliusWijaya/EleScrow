/** @type {import('tailwindcss').Config} */

const colors = require("tailwindcss/colors");

export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: colors.gray["950"],
        secondary: colors.sky["700"],
        "secondary-hover": colors.sky["800"],
        incoming: colors.emerald["800"],
        outgoing: colors.red["900"],
        "primary-text": colors.white,
        "primary-text-hover": colors.gray["200"],
        "secondary-text": colors.slate["400"],
        "incoming-text": colors.emerald["300"],
        "outgoing-text": colors.red["300"],
      },
    },
  },
  plugins: [],
};
