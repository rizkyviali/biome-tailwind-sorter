/**
 * Utility functions for extracting and manipulating class attributes in AST nodes
 */

export interface ClassAttribute {
  value: string;
  startPos: number;
  endPos: number;
  isMultiline: boolean;
  quotes: '"' | "'" | 'none';
}

/**
 * Extract class names from a class attribute string, handling multi-line classes
 */
export function extractClassNames(classString: string): string[] {
  return classString
    .trim()
    .split(/\s+/)
    .filter(cls => cls.length > 0);
}

/**
 * Reconstruct class string from array of class names, preserving original formatting
 */
export function reconstructClassString(
  classNames: string[], 
  originalString: string,
  preserveMultiline: boolean = true
): string {
  if (!preserveMultiline || !originalString.includes('\n')) {
    return classNames.join(' ');
  }
  
  // For multiline, try to preserve the original formatting structure
  const lines = originalString.split('\n');
  if (lines.length <= 1) {
    return classNames.join(' ');
  }
  
  // Simple heuristic: distribute classes evenly across lines
  const classesPerLine = Math.ceil(classNames.length / lines.length);
  const result = [];
  
  for (let i = 0; i < lines.length; i++) {
    const lineStart = i * classesPerLine;
    const lineEnd = Math.min((i + 1) * classesPerLine, classNames.length);
    const lineClasses = classNames.slice(lineStart, lineEnd);
    
    if (lineClasses.length > 0) {
      const indent = lines[i].match(/^\s*/)?.[0] || '';
      result.push(indent + lineClasses.join(' '));
    }
  }
  
  return result.join('\n');
}

/**
 * Parse class attribute from various AST node types
 */
export function parseClassAttribute(attributeValue: string, attributeName: string): ClassAttribute | null {
  if (!['class', 'className'].includes(attributeName)) {
    return null;
  }
  
  // Determine quote type
  let quotes: '"' | "'" | 'none' = 'none';
  let cleanValue = attributeValue;
  
  if (attributeValue.startsWith('"') && attributeValue.endsWith('"')) {
    quotes = '"';
    cleanValue = attributeValue.slice(1, -1);
  } else if (attributeValue.startsWith("'") && attributeValue.endsWith("'")) {
    quotes = "'";
    cleanValue = attributeValue.slice(1, -1);
  }
  
  const isMultiline = cleanValue.includes('\n') || cleanValue.includes('\\n');
  
  return {
    value: cleanValue,
    startPos: 0, // Will be set by the caller
    endPos: cleanValue.length, // Length of the clean value
    isMultiline,
    quotes
  };
}

/**
 * Check if a string is likely a Tailwind CSS class
 */
export function isTailwindClass(className: string): boolean {
  // Check if it's a known exact Tailwind class first
  const exactTailwindClasses = [
    'container', 'flex', 'grid', 'block', 'inline', 'hidden', 'visible', 'invisible',
    'absolute', 'relative', 'fixed', 'static', 'sticky'
  ];
  
  // Remove modifiers to get base class
  const baseClass = className.split(':').pop() || '';
  
  if (exactTailwindClasses.includes(baseClass)) {
    return true;
  }
  
  // Basic heuristics for Tailwind classes
  const tailwindPatterns = [
    /^(p|px|py|pt|pr|pb|pl)-/,
    /^m-\d+/,
    /^(mx|mt|mr|mb|ml)-/,
    /^my-\d+/,
    /^(w|h|min-w|min-h|max-w|max-h)-/,
    /^(text|font|leading|tracking)-/,
    /^(bg|border|rounded)-/,
    /^(top|right|bottom|left|inset)-/,
    /^(z)-/,
    /^(opacity|shadow)-/,
    /^(hover|focus|active|disabled):/,
    /^(sm|md|lg|xl|2xl):/,
    /^(dark|light):/,
    /^justify-/,
    /^items-/,
  ];
  
  return tailwindPatterns.some(pattern => pattern.test(className));
}

/**
 * Filter out non-Tailwind classes from a list
 */
export function filterTailwindClasses(classNames: string[]): string[] {
  return classNames.filter(isTailwindClass);
}

/**
 * Check if a class list contains any Tailwind classes
 */
export function containsTailwindClasses(classNames: string[]): boolean {
  return classNames.some(isTailwindClass);
}