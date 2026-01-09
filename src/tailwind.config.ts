import type { Config } from 'tailwindcss';

export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        background: '#fdfbf7', // Warm Paper
        foreground: '#2d2d2d', // Soft Pencil Black
        muted: '#e5e0d8', // Old Paper / Erased Pencil
        accent: '#ff4d4d', // Red Correction Marker
        blue: '#2d5da1', // Blue Ballpoint Pen
        'post-it': '#fff9c4', // Post-it Yellow
      },
      fontFamily: {
        sans: ['Patrick Hand', 'sans-serif'],
        heading: ['Kalam', 'cursive'],
      },
      borderRadius: {
        'wobbly': '255px 15px 225px 15px / 15px 225px 15px 255px',
        'wobbly-md': '20px 225px 20px 225px / 225px 20px 225px 20px',
      },
      boxShadow: {
        'hard': '4px 4px 0px 0px #2d2d2d',
        'hard-hover': '2px 2px 0px 0px #2d2d2d',
        'hard-xl': '8px 8px 0px 0px #2d2d2d',
      },
      rotate: {
        '1': '1deg',
        '-1': '-1deg',
        '2': '2deg',
        '-2': '-2deg',
      }
    },
  },
  plugins: [],
} satisfies Config;
