/**
 * Biome formatter plugin for Tailwind CSS class sorting
 * This integrates with Biome's formatting pipeline to automatically sort classes
 */

import { sortTailwindClasses } from './tailwind-order';
import { extractClassNames, parseClassAttribute, reconstructClassString, containsTailwindClasses } from './class-extractor';

export interface BiomeFormatContext {
  getText(range?: [number, number]): string;
  replaceText(range: [number, number], text: string): void;
}

export interface BiomeFormatNode {
  type: string;
  start: number;
  end: number;
  value?: string;
  attributes?: Array<{
    name: string;
    value: string;
    start: number;
    end: number;
  }>;
  children?: BiomeFormatNode[];
}

/**
 * Main formatter class that processes AST nodes and sorts Tailwind classes
 */
export class TailwindFormatter {
  /**
   * Format a single AST node and its children
   */
  static formatNode(node: BiomeFormatNode, context: BiomeFormatContext): void {
    // Process current node
    if (this.shouldProcessNode(node)) {
      this.processElementNode(node, context);
    }

    // Recursively process children
    if (node.children) {
      for (const child of node.children) {
        this.formatNode(child, context);
      }
    }
  }

  /**
   * Check if a node should be processed for class sorting
   */
  private static shouldProcessNode(node: BiomeFormatNode): boolean {
    return ['JSXElement', 'HTMLElement', 'VueElement'].includes(node.type) && 
           !!node.attributes && 
           node.attributes.length > 0;
  }

  /**
   * Process an element node and sort its class attributes
   */
  private static processElementNode(node: BiomeFormatNode, context: BiomeFormatContext): void {
    if (!node.attributes) return;

    for (const attribute of node.attributes) {
      if (!['class', 'className'].includes(attribute.name)) {
        continue;
      }

      const attributeText = context.getText([attribute.start, attribute.end]);
      const classAttr = parseClassAttribute(attribute.value, attribute.name);
      
      if (!classAttr) continue;

      const classNames = extractClassNames(classAttr.value);
      
      // Only process if we have Tailwind classes
      if (!containsTailwindClasses(classNames)) {
        continue;
      }

      const sortedClasses = sortTailwindClasses(classNames);
      
      // Check if sorting is needed
      if (JSON.stringify(classNames) === JSON.stringify(sortedClasses)) {
        continue;
      }

      // Reconstruct the sorted class string
      const sortedClassString = reconstructClassString(
        sortedClasses,
        classAttr.value,
        classAttr.isMultiline
      );

      // Build the new attribute value
      let newAttributeValue: string;
      if (classAttr.quotes === '"') {
        newAttributeValue = `${attribute.name}="${sortedClassString}"`;
      } else if (classAttr.quotes === "'") {
        newAttributeValue = `${attribute.name}='${sortedClassString}'`;
      } else {
        newAttributeValue = `${attribute.name}=${sortedClassString}`;
      }

      // Replace the attribute in the source
      context.replaceText([attribute.start, attribute.end], newAttributeValue);
    }
  }
}

/**
 * Main format function that processes the entire document
 */
export function formatDocument(source: string): string {
  // For now, we'll use a simple regex-based approach that works without full AST parsing
  // This is a transitional solution until Biome's plugin system is fully available
  
  return source.replace(
    /(class(?:Name)?=["']?)([^"']*)(["']?)/g, 
    (match, prefix, classString, suffix) => {
      const classNames = extractClassNames(classString);
      
      if (!containsTailwindClasses(classNames)) {
        return match;
      }

      const sortedClasses = sortTailwindClasses(classNames);
      
      // Check if sorting is needed
      if (JSON.stringify(classNames) === JSON.stringify(sortedClasses)) {
        return match;
      }

      const sortedClassString = reconstructClassString(sortedClasses, classString, classString.includes('\n'));
      return `${prefix}${sortedClassString}${suffix}`;
    }
  );
}

/**
 * Export for programmatic usage
 */
export const formatter = {
  name: 'tailwind-sorter',
  format: formatDocument,
  supports: ['javascript', 'typescript', 'jsx', 'tsx', 'html', 'vue']
};