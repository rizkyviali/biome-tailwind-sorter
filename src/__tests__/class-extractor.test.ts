/**
 * Tests for class extraction and manipulation functionality
 */

import {
  extractClassNames,
  reconstructClassString,
  parseClassAttribute,
  isTailwindClass,
  filterTailwindClasses,
  containsTailwindClasses
} from '../class-extractor';

describe('extractClassNames', () => {
  test('extracts single line classes', () => {
    const input = 'p-4 bg-red-500 text-white';
    const expected = ['p-4', 'bg-red-500', 'text-white'];
    expect(extractClassNames(input)).toEqual(expected);
  });

  test('extracts multi-line classes', () => {
    const input = `
      p-4
      bg-red-500
      text-white
    `;
    const expected = ['p-4', 'bg-red-500', 'text-white'];
    expect(extractClassNames(input)).toEqual(expected);
  });

  test('handles extra whitespace', () => {
    const input = '   p-4    bg-red-500     text-white   ';
    const expected = ['p-4', 'bg-red-500', 'text-white'];
    expect(extractClassNames(input)).toEqual(expected);
  });

  test('handles empty string', () => {
    const input = '';
    const expected: string[] = [];
    expect(extractClassNames(input)).toEqual(expected);
  });

  test('handles mixed whitespace and newlines', () => {
    const input = 'p-4\n  bg-red-500   \n text-white';
    const expected = ['p-4', 'bg-red-500', 'text-white'];
    expect(extractClassNames(input)).toEqual(expected);
  });
});

describe('reconstructClassString', () => {
  test('reconstructs single line classes', () => {
    const classNames = ['p-4', 'bg-red-500', 'text-white'];
    const original = 'text-white bg-red-500 p-4';
    const result = reconstructClassString(classNames, original, false);
    expect(result).toBe('p-4 bg-red-500 text-white');
  });

  test('preserves multiline format when requested', () => {
    const classNames = ['p-4', 'bg-red-500', 'text-white', 'rounded-lg'];
    const original = `
      text-white
      bg-red-500
    `;
    const result = reconstructClassString(classNames, original, true);
    expect(result).toContain('\n');
  });

  test('falls back to single line for non-multiline original', () => {
    const classNames = ['p-4', 'bg-red-500', 'text-white'];
    const original = 'text-white bg-red-500 p-4';
    const result = reconstructClassString(classNames, original, true);
    expect(result).toBe('p-4 bg-red-500 text-white');
  });
});

describe('parseClassAttribute', () => {
  test('parses double-quoted class attribute', () => {
    const result = parseClassAttribute('"p-4 bg-red-500"', 'className');
    expect(result).toEqual({
      value: 'p-4 bg-red-500',
      startPos: 0,
      endPos: 14,
      isMultiline: false,
      quotes: '"'
    });
  });

  test('parses single-quoted class attribute', () => {
    const result = parseClassAttribute("'p-4 bg-red-500'", 'class');
    expect(result).toEqual({
      value: 'p-4 bg-red-500',
      startPos: 0,
      endPos: 14,
      isMultiline: false,
      quotes: "'"
    });
  });

  test('parses unquoted class attribute', () => {
    const result = parseClassAttribute('p-4 bg-red-500', 'className');
    expect(result).toEqual({
      value: 'p-4 bg-red-500',
      startPos: 0,
      endPos: 14,
      isMultiline: false,
      quotes: 'none'
    });
  });

  test('detects multiline classes', () => {
    const multilineClass = '"p-4\\nbg-red-500\\ntext-white"';
    const result = parseClassAttribute(multilineClass, 'className');
    expect(result?.isMultiline).toBe(true);
  });

  test('returns null for non-class attributes', () => {
    const result = parseClassAttribute('"some-value"', 'id');
    expect(result).toBeNull();
  });
});

describe('isTailwindClass', () => {
  test('identifies common Tailwind classes', () => {
    const tailwindClasses = [
      'p-4',
      'mt-8',
      'bg-red-500',
      'text-white',
      'flex',
      'justify-center',
      'hover:bg-blue-500',
      'md:text-lg',
      'dark:bg-gray-800'
    ];

    tailwindClasses.forEach(cls => {
      expect(isTailwindClass(cls)).toBe(true);
    });
  });

  test('rejects non-Tailwind classes', () => {
    const nonTailwindClasses = [
      'my-custom-class',
      'bootstrap-btn',
      'container-fluid',
      'nav-item'
    ];

    nonTailwindClasses.forEach(cls => {
      expect(isTailwindClass(cls)).toBe(false);
    });
  });
});

describe('filterTailwindClasses', () => {
  test('filters out non-Tailwind classes', () => {
    const mixed = ['p-4', 'custom-class', 'bg-red-500', 'bootstrap-btn', 'text-white'];
    const expected = ['p-4', 'bg-red-500', 'text-white'];
    expect(filterTailwindClasses(mixed)).toEqual(expected);
  });

  test('returns empty array when no Tailwind classes', () => {
    const nonTailwind = ['custom-class', 'bootstrap-btn'];
    expect(filterTailwindClasses(nonTailwind)).toEqual([]);
  });
});

describe('containsTailwindClasses', () => {
  test('returns true when Tailwind classes are present', () => {
    const classes = ['custom-class', 'p-4', 'bootstrap-btn'];
    expect(containsTailwindClasses(classes)).toBe(true);
  });

  test('returns false when no Tailwind classes are present', () => {
    const classes = ['custom-class', 'bootstrap-btn'];
    expect(containsTailwindClasses(classes)).toBe(false);
  });

  test('returns false for empty array', () => {
    expect(containsTailwindClasses([])).toBe(false);
  });
});