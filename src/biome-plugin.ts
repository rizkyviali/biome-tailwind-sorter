/**
 * Biome plugin implementation for Tailwind CSS class sorting
 * This creates a formatter plugin that integrates with Biome's formatting system
 */

import { formatDocument } from './tailwind-formatter';

export interface BiomePluginInterface {
  name: string;
  version: string;
  formatters?: Record<string, BiomeFormatter>;
  transformers?: Record<string, BiomeTransformer>;
}

export interface BiomeFormatter {
  name: string;
  format(source: string, options?: any): string;
  canFormat(filePath: string): boolean;
}

export interface BiomeTransformer {
  name: string;
  transform(source: string, options?: any): string;
  canTransform(filePath: string): boolean;
}

/**
 * Tailwind class sorter formatter for Biome
 */
class TailwindSorterFormatter implements BiomeFormatter {
  name = 'tailwind-sorter';

  /**
   * Format source code by sorting Tailwind classes
   */
  format(source: string, options?: any): string {
    try {
      return formatDocument(source);
    } catch (error) {
      console.warn('Tailwind class sorting failed:', error);
      return source; // Return original source if formatting fails
    }
  }

  /**
   * Check if this formatter can handle the given file
   */
  canFormat(filePath: string): boolean {
    const supportedExtensions = ['.js', '.jsx', '.ts', '.tsx', '.html', '.vue'];
    return supportedExtensions.some(ext => filePath.endsWith(ext));
  }
}

/**
 * Tailwind transformer that works as a post-processing step
 */
class TailwindSorterTransformer implements BiomeTransformer {
  name = 'tailwind-sorter-transform';

  /**
   * Transform source code by sorting Tailwind classes
   */
  transform(source: string, options?: any): string {
    try {
      return formatDocument(source);
    } catch (error) {
      console.warn('Tailwind class transformation failed:', error);
      return source;
    }
  }

  /**
   * Check if this transformer can handle the given file
   */
  canTransform(filePath: string): boolean {
    const supportedExtensions = ['.js', '.jsx', '.ts', '.tsx', '.html', '.vue'];
    return supportedExtensions.some(ext => filePath.endsWith(ext));
  }
}

/**
 * Main Biome plugin export
 */
export const biomeTailwindSorterPlugin: BiomePluginInterface = {
  name: 'biome-tailwind-sorter',
  version: '1.1.0',
  formatters: {
    'tailwind-sorter': new TailwindSorterFormatter(),
  },
  transformers: {
    'tailwind-sorter': new TailwindSorterTransformer(),
  },
};

/**
 * Plugin factory function for dynamic loading
 */
export function createBiomePlugin(): BiomePluginInterface {
  return biomeTailwindSorterPlugin;
}

/**
 * Default export for CommonJS compatibility
 */
export default biomeTailwindSorterPlugin;