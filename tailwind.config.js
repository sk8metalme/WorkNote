/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './src/**/*.{html,js,svelte,ts}',
  ],
  theme: {
    extend: {
      fontFamily: {
        'line-seed': ['"LINE Seed JP"', 'sans-serif'],
      },
      colors: {
        'ly-green': '#06C755',
        'ly-red': '#FF0033',
        'ly-navy': '#000048',
        'ly-gray': {
          50: '#F7F8FA',
          100: '#E9ECF0',
          200: '#D1D6DE',
          300: '#B0B8C4',
          400: '#8A95A5',
          500: '#6B7684',
          600: '#515A68',
          700: '#3D4552',
          800: '#2A303A',
          900: '#1A1E26',
        },
      },
    },
  },
  plugins: [require('@tailwindcss/typography')],
}

