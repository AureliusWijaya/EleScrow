// tailwind.config.js
const { fontFamily } = require('tailwindcss/defaultTheme');

module.exports = {
  darkMode: ['class'],
  content: [
    './index.html',
    './src/**/*.{js,ts,jsx,tsx}',
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', ...fontFamily.sans],
      },
      borderRadius: {
        lg: 'var(--radius)',
        md: 'calc(var(--radius) - 2px)',
        sm: 'calc(var(--radius) - 4px)',
      },
      colors: {
        dashboardTheme: {
          background: '#1F2937',   // gray-800
          surface: '#111827',      // gray-900
          border: '#374151',       // gray-700
          text: '#E5E7EB',         // gray-200
          muted: '#9CA3AF',        // gray-400
          accent: '#60A5FA',       // blue-400
        },
        blackTheme: {
          background: '#000000',
          surface: '#1A1A1A',
          border: '#333333',
          text: '#FFFFFF',
          muted: '#AAAAAA',
          accent: '#F472B6',       // pink-400
        },
        creamTheme: {
          background: '#FDF6EC',   // soft cream
          surface: '#FFFFFF',
          border: '#E5E7EB',       // gray-200
          text: '#1F2937',         // gray-800
          muted: '#6B7280',        // gray-500
          accent: '#FBBF24',       // amber-400
        },
      },
      keyframes: {
        'accordion-down': {
          from: { height: 0 },
          to: { height: 'var(--radix-accordion-content-height)' },
        },
        'accordion-up': {
          from: { height: 'var(--radix-accordion-content-height)' },
          to: { height: 0 },
        },
      },
      animation: {
        'accordion-down': 'accordion-down 0.2s ease-out',
        'accordion-up': 'accordion-up 0.2s ease-out',
      },
    },
  },
  plugins: [
    require('tailwindcss-animate'),
    require('@tailwindcss/forms'),
  ],
};