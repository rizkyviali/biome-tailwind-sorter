import React from 'react';

// Before: Classes are unsorted
function UnsortedButton() {
  return (
    <button
      className="
        text-white
        bg-blue-500
        p-4
        hover:bg-blue-600
        rounded-lg
        font-semibold
        transition-colors
        duration-200
        focus:outline-none
        focus:ring-2
        focus:ring-blue-500
        focus:ring-offset-2
        disabled:opacity-50
        disabled:cursor-not-allowed
      "
    >
      Click me
    </button>
  );
}

// After: Classes will be automatically sorted by the plugin
function SortedButton() {
  return (
    <button
      className="
        p-4
        font-semibold
        text-white
        bg-blue-500
        rounded-lg
        transition-colors
        duration-200
        hover:bg-blue-600
        focus:outline-none
        focus:ring-2
        focus:ring-blue-500
        focus:ring-offset-2
        disabled:cursor-not-allowed
        disabled:opacity-50
      "
    >
      Click me
    </button>
  );
}

// Single line example
function InlineButton() {
  // Before: className="text-white bg-red-500 p-4 hover:bg-red-600"
  // After: className="p-4 text-white bg-red-500 hover:bg-red-600"
  return (
    <button className="p-4 text-white bg-red-500 hover:bg-red-600">
      Inline Button
    </button>
  );
}

// Responsive classes example
function ResponsiveCard() {
  return (
    <div
      className="
        p-4
        w-full
        max-w-sm
        bg-white
        border
        border-gray-200
        rounded-lg
        shadow-md
        sm:p-6
        md:max-w-md
        lg:max-w-lg
        xl:max-w-xl
        hover:shadow-lg
        dark:bg-gray-800
        dark:border-gray-700
      "
    >
      <h3 className="mb-2 text-xl font-bold text-gray-900 dark:text-white">
        Card Title
      </h3>
      <p className="text-gray-700 dark:text-gray-300">
        This card demonstrates responsive Tailwind classes that are properly
        sorted by the plugin.
      </p>
    </div>
  );
}

export { UnsortedButton, SortedButton, InlineButton, ResponsiveCard };