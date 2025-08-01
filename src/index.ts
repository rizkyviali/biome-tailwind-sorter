/**
 * Biome Plugin for Tailwind CSS Class Sorting
 * 
 * This plugin provides both linter rules and formatter functionality:
 * - Formatter: Automatically sorts Tailwind classes on file save
 * - Linter rule: `tailwind/useSortedClasses` for detecting unsorted classes
 * - Supports HTML, JSX, TSX, and Vue files
 * - Preserves multi-line class definitions
 */

import { useSortedClassesRule } from './use-sorted-classes-rule';
import { biomeTailwindSorterPlugin } from './biome-plugin';
import { formatDocument } from './tailwind-formatter';

export interface BiomePlugin {
  name: string;
  version: string;
  rules?: Record<string, any>;
  formatters?: Record<string, any>;
  transformers?: Record<string, any>;
}

export interface BiomePluginConfig {
  plugins?: string[];
  linter?: {
    rules?: Record<string, any>;
  };
  formatter?: {
    enabled?: boolean;
    plugins?: string[];
  };
}

/**
 * Main plugin export for Biome (with both linter and formatter)
 */
export const plugin: BiomePlugin = {
  name: 'biome-tailwind-sorter',
  version: '1.1.0',
  rules: {
    'tailwind/useSortedClasses': useSortedClassesRule,
  },
  formatters: biomeTailwindSorterPlugin.formatters,
  transformers: biomeTailwindSorterPlugin.transformers,
};

/**
 * Formatter-only plugin export
 */
export const formatterPlugin = biomeTailwindSorterPlugin;

/**
 * Default export for CommonJS compatibility
 */
export default plugin;

/**
 * Re-export utilities for advanced usage
 */
export { sortTailwindClasses, areClassesSorted } from './tailwind-order';
export { extractClassNames, containsTailwindClasses } from './class-extractor';
export { UseSortedClassesRule } from './use-sorted-classes-rule';
export { formatDocument } from './tailwind-formatter';
export { TailwindFormatter } from './tailwind-formatter';

/**
 * Plugin metadata
 */
export const metadata = {
  name: 'biome-tailwind-sorter',
  description: 'A Biome plugin for sorting Tailwind CSS classes according to official order',
  version: '1.0.0',
  author: 'Rizky Viali',
  repository: 'https://github.com/rizkyviali/biome-tailwind-sorter',
  keywords: ['biome', 'plugin', 'tailwind', 'css', 'formatter', 'linter'],
  supportedLanguages: ['javascript', 'typescript', 'jsx', 'tsx', 'html', 'vue'],
  rules: {
    'tailwind/useSortedClasses': {
      description: 'Enforce sorted Tailwind CSS classes according to official order',
      category: 'style',
      fixable: true,
      severity: 'warning',
      examples: {
        invalid: [
          '<div className="text-white bg-red-500 p-4" />',
          '<div class="hover:bg-blue-500 bg-red-500 text-white" />',
        ],
        valid: [
          '<div className="p-4 bg-red-500 text-white" />',
          '<div class="bg-red-500 text-white hover:bg-blue-500" />',
        ],
      },
    },
  },
};