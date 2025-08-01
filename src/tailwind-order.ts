/**
 * Tailwind CSS class ordering based on official documentation
 * https://tailwindcss.com/blog/automatic-class-sorting-with-prettier#how-classes-are-sorted
 */

export interface TailwindClass {
  name: string;
  order: number;
  modifier?: string;
}

// Tailwind class order categories
const TAILWIND_ORDER_MAP: Record<string, number> = {
  // Layout
  'container': 0,
  'box-border': 10,
  'box-content': 10,
  'block': 20,
  'inline-block': 20,
  'inline': 20,
  'flex': 20,
  'inline-flex': 20,
  'table': 20,
  'inline-table': 20,
  'table-caption': 20,
  'table-cell': 20,
  'table-column': 20,
  'table-column-group': 20,
  'table-footer-group': 20,
  'table-header-group': 20,
  'table-row-group': 20,
  'table-row': 20,
  'flow-root': 20,
  'grid': 20,
  'inline-grid': 20,
  'contents': 20,
  'list-item': 20,
  'hidden': 20,
  
  // Position
  'static': 30,
  'fixed': 30,
  'absolute': 30,
  'relative': 30,
  'sticky': 30,
  
  // Top/Right/Bottom/Left
  'inset': 40,
  'top': 40,
  'right': 40,
  'bottom': 40,
  'left': 40,
  
  // Visibility
  'visible': 50,
  'invisible': 50,
  'collapse': 50,
  
  // Z-Index
  'z': 60,
  
  // Flex and Grid
  'flex-row': 70,
  'flex-row-reverse': 70,
  'flex-col': 70,
  'flex-col-reverse': 70,
  'flex-wrap': 70,
  'flex-wrap-reverse': 70,
  'flex-nowrap': 70,
  'place-content': 70,
  'place-items': 70,
  'align-content': 70,
  'align-items': 70,
  'align-self': 70,
  'justify-content': 70,
  'justify-items': 70,
  'justify-self': 70,
  'flex-auto': 70,
  'flex-initial': 70,
  'flex-none': 70,
  'grow': 70,
  'shrink': 70,
  'order': 70,
  'grid-cols': 70,
  'col-auto': 70,
  'col-span': 70,
  'col-start': 70,
  'col-end': 70,
  'grid-rows': 70,
  'row-auto': 70,
  'row-span': 70,
  'row-start': 70,
  'row-end': 70,
  'gap': 70,
  
  // Spacing (Padding and Margin)
  'p': 80,
  'px': 80,
  'py': 80,
  'pt': 80,
  'pr': 80,
  'pb': 80,
  'pl': 80,
  'm': 80,
  'mx': 80,
  'my': 80,
  'mt': 80,
  'mr': 80,
  'mb': 80,
  'ml': 80,
  'space-x': 80,
  'space-y': 80,
  
  // Sizing
  'w': 90,
  'min-w': 90,
  'max-w': 90,
  'h': 90,
  'min-h': 90,
  'max-h': 90,
  
  // Typography
  'font-family': 100,
  'font-size': 100,
  'font-smoothing': 100,
  'font-style': 100,
  'font-weight': 100,
  'font-variant-numeric': 100,
  'letter-spacing': 100,
  'line-height': 100,
  'list-style-image': 100,
  'list-style-position': 100,
  'list-style-type': 100,
  'text-align': 100,
  'text-color': 100,
  'text-decoration': 100,
  'text-decoration-color': 100,
  'text-decoration-style': 100,
  'text-decoration-thickness': 100,
  'text-underline-offset': 100,
  'text-transform': 100,
  'text-overflow': 100,
  'vertical-align': 100,
  'whitespace': 100,
  'word-break': 100,
  'hyphens': 100,
  'content': 100,
  
  // Backgrounds
  'bg': 110,
  'from': 110,
  'via': 110,
  'to': 110,
  'bg-attachment': 110,
  'bg-clip': 110,
  'bg-origin': 110,
  'bg-position': 110,
  'bg-repeat': 110,
  'bg-size': 110,
  'bg-image': 110,
  
  // Borders
  'border': 120,
  'border-collapse': 120,
  'border-spacing': 120,
  'table-layout': 120,
  'border-style': 120,
  'divide': 120,
  'outline': 120,
  'ring': 120,
  
  // Effects
  'shadow': 130,
  'opacity': 130,
  'mix-blend-mode': 130,
  'bg-blend-mode': 130,
  
  // Filters
  'filter': 140,
  'blur': 140,
  'brightness': 140,
  'contrast': 140,
  'drop-shadow': 140,
  'grayscale': 140,
  'hue-rotate': 140,
  'invert': 140,
  'saturate': 140,
  'sepia': 140,
  'backdrop-filter': 140,
  'backdrop-blur': 140,
  'backdrop-brightness': 140,
  'backdrop-contrast': 140,
  'backdrop-grayscale': 140,
  'backdrop-hue-rotate': 140,
  'backdrop-invert': 140,
  'backdrop-opacity': 140,
  'backdrop-saturate': 140,
  'backdrop-sepia': 140,
  
  // Tables
  'caption-side': 150,
  'empty-cells': 150,
  
  // Transitions and Animation
  'transition': 160,
  'duration': 160,
  'ease': 160,
  'delay': 160,
  'animate': 160,
  
  // Transforms
  'transform': 170,
  'transform-origin': 170,
  'scale': 170,
  'rotate': 170,
  'translate': 170,
  'skew': 170,
  
  // Interactivity
  'accent-color': 180,
  'appearance': 180,
  'cursor': 180,
  'caret-color': 180,
  'pointer-events': 180,
  'resize': 180,
  'scroll-behavior': 180,
  'scroll-margin': 180,
  'scroll-padding': 180,
  'scroll-snap-align': 180,
  'scroll-snap-stop': 180,
  'scroll-snap-type': 180,
  'touch-action': 180,
  'user-select': 180,
  'will-change': 180,
  
  // SVG
  'fill': 190,
  'stroke': 190,
  
  // Accessibility
  'sr-only': 200,
  'not-sr-only': 200,
  
  // Official
  'forced-color-adjust': 210,
};

/**
 * Parse a Tailwind class to extract base class and modifiers
 */
export function parseTailwindClass(className: string): TailwindClass {
  const parts = className.split(':');
  const actualClass = parts[parts.length - 1];
  const modifiers = parts.slice(0, -1).join(':');
  
  // Handle arbitrary values like w-[100px]
  const baseClass = actualClass.replace(/\[.*?\]$/, '');
  
  // Handle numeric suffixes like p-4, text-lg, etc.
  const basePattern = baseClass.replace(/-\d+(\.\d+)?$/, '').replace(/-[a-z]+$/, '');
  
  const order = getClassOrder(basePattern, actualClass);
  
  return {
    name: className,
    order,
    modifier: modifiers || undefined
  };
}

/**
 * Get the sorting order for a Tailwind class
 */
function getClassOrder(basePattern: string, fullClass: string): number {
  // Check exact match first
  if (TAILWIND_ORDER_MAP[fullClass]) {
    return TAILWIND_ORDER_MAP[fullClass];
  }
  
  // Check base pattern
  if (TAILWIND_ORDER_MAP[basePattern]) {
    return TAILWIND_ORDER_MAP[basePattern];
  }
  
  // Check for common patterns
  for (const [pattern, order] of Object.entries(TAILWIND_ORDER_MAP)) {
    if (fullClass.startsWith(pattern + '-') || fullClass === pattern) {
      return order;
    }
  }
  
  // Handle special cases
  if (fullClass.startsWith('text-')) {
    if (fullClass.match(/^text-(xs|sm|base|lg|xl|\d+xl)$/)) {
      return 100; // font-size
    }
    if (fullClass.match(/^text-(left|center|right|justify|start|end)$/)) {
      return 100; // text-align
    }
    return 100; // text-color by default
  }
  
  if (fullClass.startsWith('bg-')) {
    return 110;
  }
  
  if (fullClass.startsWith('border-')) {
    return 120;
  }
  
  if (fullClass.startsWith('rounded')) {
    return 120;
  }
  
  if (fullClass.startsWith('shadow')) {
    return 130;
  }
  
  if (fullClass.startsWith('font-')) {
    return 100;
  }
  
  if (fullClass.startsWith('transition')) {
    return 160;
  }
  
  if (fullClass.startsWith('duration')) {
    return 160;
  }
  
  // Default to high number for unknown classes
  return 999;
}

/**
 * Sort Tailwind classes according to official order
 */
export function sortTailwindClasses(classes: string[]): string[] {
  const parsedClasses = classes.map(parseTailwindClass);
  
  return parsedClasses
    .sort((a, b) => {
      // First sort by order
      if (a.order !== b.order) {
        return a.order - b.order;
      }
      
      // Then by modifier (responsive, pseudo-classes, etc.)
      const aModifier = a.modifier || '';
      const bModifier = b.modifier || '';
      
      if (aModifier !== bModifier) {
        // Sort base classes first (no modifier)
        if (!aModifier) return -1;
        if (!bModifier) return 1;
        
        // Sort responsive modifiers in order: sm, md, lg, xl, 2xl
        const responsiveOrder = ['sm', 'md', 'lg', 'xl', '2xl'];
        const aResponsive = responsiveOrder.indexOf(aModifier);
        const bResponsive = responsiveOrder.indexOf(bModifier);
        
        if (aResponsive !== -1 && bResponsive !== -1) {
          return aResponsive - bResponsive;
        }
        
        return aModifier.localeCompare(bModifier);
      }
      
      // Finally by class name alphabetically
      return a.name.localeCompare(b.name);
    })
    .map(c => c.name);
}

/**
 * Check if classes are already sorted
 */
export function areClassesSorted(classes: string[]): boolean {
  const sorted = sortTailwindClasses(classes);
  return JSON.stringify(classes) === JSON.stringify(sorted);
}