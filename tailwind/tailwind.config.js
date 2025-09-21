/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: ["../templates/**/*.{html,js}"],
  safelist: [
    'dark:bg-gray-900',
    'dark:bg-gray-800',
    'dark:bg-gray-700',
    'dark:text-white',
    'dark:text-gray-100',
    'dark:text-gray-400',
    'dark:border-gray-700',
    'dark:prose-invert'
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}