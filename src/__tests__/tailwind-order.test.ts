/**
 * Tests for Tailwind class ordering functionality
 */

import { sortTailwindClasses, areClassesSorted, parseTailwindClass } from '../tailwind-order';

describe('sortTailwindClasses', () => {
  test('sorts basic layout and spacing classes correctly', () => {
    const input = ['text-white', 'bg-red-500', 'p-4'];
    const expected = ['p-4', 'text-white', 'bg-red-500'];
    expect(sortTailwindClasses(input)).toEqual(expected);
  });

  test('sorts responsive classes correctly', () => {
    const input = ['lg:p-8', 'md:p-6', 'p-4'];
    const expected = ['p-4', 'md:p-6', 'lg:p-8'];
    expect(sortTailwindClasses(input)).toEqual(expected);
  });

  test('sorts pseudo-classes correctly', () => {
    const input = ['hover:bg-blue-500', 'bg-red-500', 'focus:bg-green-500'];
    const expected = ['bg-red-500', 'focus:bg-green-500', 'hover:bg-blue-500'];
    expect(sortTailwindClasses(input)).toEqual(expected);
  });

  test('sorts flex classes correctly', () => {
    const input = ['justify-center', 'items-center', 'flex'];
    const expected = ['flex', 'items-center', 'justify-center'];
    expect(sortTailwindClasses(input)).toEqual(expected);
  });

  test('sorts complex combination correctly', () => {
    const input = [
      'duration-300',
      'transition-all',
      'hover:shadow-lg',
      'bg-blue-500',
      'text-white',
      'py-2',
      'px-4',
      'rounded-lg',
      'font-semibold'
    ];
    const expected = [
      'px-4',
      'py-2',
      'font-semibold',
      'text-white',
      'bg-blue-500',
      'rounded-lg',
      'hover:shadow-lg',
      'duration-300',
      'transition-all'
    ];
    expect(sortTailwindClasses(input)).toEqual(expected);
  });

  test('handles arbitrary values correctly', () => {
    const input = ['w-[100px]', 'h-[50px]', 'bg-[#ff0000]'];
    const expected = ['h-[50px]', 'w-[100px]', 'bg-[#ff0000]'];
    expect(sortTailwindClasses(input)).toEqual(expected);
  });
});

describe('areClassesSorted', () => {
  test('returns true for correctly sorted classes', () => {
    const classes = ['p-4', 'text-white', 'bg-red-500'];
    expect(areClassesSorted(classes)).toBe(true);
  });

  test('returns false for incorrectly sorted classes', () => {
    const classes = ['text-white', 'bg-red-500', 'p-4'];
    expect(areClassesSorted(classes)).toBe(false);
  });

  test('returns true for single class', () => {
    const classes = ['p-4'];
    expect(areClassesSorted(classes)).toBe(true);
  });

  test('returns true for empty array', () => {
    const classes: string[] = [];
    expect(areClassesSorted(classes)).toBe(true);
  });
});

describe('parseTailwindClass', () => {
  test('parses simple class correctly', () => {
    const result = parseTailwindClass('p-4');
    expect(result.name).toBe('p-4');
    expect(result.order).toBe(80); // spacing
    expect(result.modifier).toBeUndefined();
  });

  test('parses class with responsive modifier', () => {
    const result = parseTailwindClass('md:p-4');
    expect(result.name).toBe('md:p-4');
    expect(result.order).toBe(80); // spacing
    expect(result.modifier).toBe('md');
  });

  test('parses class with multiple modifiers', () => {
    const result = parseTailwindClass('lg:hover:bg-blue-500');
    expect(result.name).toBe('lg:hover:bg-blue-500');
    expect(result.modifier).toBe('lg:hover');
  });

  test('parses arbitrary value class', () => {
    const result = parseTailwindClass('w-[100px]');
    expect(result.name).toBe('w-[100px]');
    expect(result.order).toBe(90); // sizing
  });
});