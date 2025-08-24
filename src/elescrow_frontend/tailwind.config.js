/** @type {import('tailwindcss').Config} */

const colors = require("tailwindcss/colors");

export default {
    content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
    theme: {
        extend: {
            height: {
                page: "calc(100vh - 80px)",
            },
            minHeight: {
                page: "calc(100vh - 80px)",
            },
            colors: {
                primary: colors.gray["950"],
                "primary-hover": colors.gray["900"],
                secondary: colors.sky["700"],
                "secondary-hover": colors.sky["800"],
                tertiary: colors.slate["900"],
                error: colors.red["400"],
                icp: colors.violet["900"],
                "icp-hover": colors.violet["800"],
                incoming: colors.emerald["800"],
                outgoing: colors.red["900"],
                "primary-text": colors.white,
                "primary-text-hover": colors.gray["200"],
                "secondary-text": colors.slate["400"],
                "incoming-text": colors.emerald["300"],
                "outgoing-text": colors.red["300"],
            },
            keyframes: {
                fadeIn: {
                    "0%": { opacity: 0 },
                    "100%": { opacity: 1 },
                },
                fadeOut: {
                    "0%": { opacity: 1 },
                    "100%": { opacity: 0 },
                },
            },
            animation: {
                "fade-in": "fadeIn 200ms ease-in-out",
                "fade-out": "fadeOut 200ms ease-in-out",
            },
        },
    },
    plugins: [],
};
