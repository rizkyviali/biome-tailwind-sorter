/**
 * Biome rule implementation for sorting Tailwind CSS classes
 */

import { sortTailwindClasses, areClassesSorted } from './tailwind-order';
import { 
  extractClassNames, 
  reconstructClassString, 
  parseClassAttribute, 
  containsTailwindClasses 
} from './class-extractor';

export interface BiomeRuleContext {
  report(diagnostic: BiomeDiagnostic): void;
}

export interface BiomeDiagnostic {
  message: string;
  node: any;
  fix?: BiomeFix;
}

export interface BiomeFix {
  title: string;
  edit: BiomeEdit;
}

export interface BiomeEdit {
  range: [number, number];
  text: string;
}

export interface BiomeNode {
  type: string;
  value?: string;
  name?: string;
  attributes?: BiomeAttribute[];
  children?: BiomeNode[];
  range: [number, number];
}

export interface BiomeAttribute {
  name: string;
  value: string;
  range: [number, number];
}

/**
 * Main rule implementation for sorting Tailwind classes
 */
export class UseSortedClassesRule {
  static ruleName = 'tailwind/useSortedClasses';
  static category = 'style';
  static description = 'Enforce sorted Tailwind CSS classes';
  static fixable = true;

  /**
   * Check if a node has class or className attributes that need sorting
   */
  private checkNode(node: BiomeNode, context: BiomeRuleContext): void {
    if (!node.attributes) {
      return;
    }

    for (const attribute of node.attributes) {
      if (!['class', 'className'].includes(attribute.name)) {
        continue;
      }

      const classAttr = parseClassAttribute(attribute.value, attribute.name);
      if (!classAttr) {
        continue;
      }

      const classNames = extractClassNames(classAttr.value);
      
      // Only process if we have Tailwind classes
      if (!containsTailwindClasses(classNames)) {
        continue;
      }

      if (!areClassesSorted(classNames)) {
        this.reportUnsortedClasses(node, attribute, classNames, context);
      }
    }
  }

  /**
   * Report unsorted classes with auto-fix
   */
  private reportUnsortedClasses(
    node: BiomeNode,
    attribute: BiomeAttribute,
    classNames: string[],
    context: BiomeRuleContext
  ): void {
    const sortedClasses = sortTailwindClasses(classNames);
    const classAttr = parseClassAttribute(attribute.value, attribute.name)!;
    
    const sortedClassString = reconstructClassString(
      sortedClasses,
      classAttr.value,
      classAttr.isMultiline
    );

    // Construct the full attribute value with quotes
    let newAttributeValue: string;
    if (classAttr.quotes === '"') {
      newAttributeValue = `"${sortedClassString}"`;
    } else if (classAttr.quotes === "'") {
      newAttributeValue = `'${sortedClassString}'`;
    } else {
      newAttributeValue = sortedClassString;
    }

    const diagnostic: BiomeDiagnostic = {
      message: `Tailwind CSS classes should be sorted according to the official order. Expected: ${sortedClasses.join(' ')}`,
      node: node,
      fix: {
        title: 'Sort Tailwind CSS classes',
        edit: {
          range: attribute.range,
          text: `${attribute.name}=${newAttributeValue}`
        }
      }
    };

    context.report(diagnostic);
  }

  /**
   * Visit JSX Element nodes
   */
  visitJSXElement(node: BiomeNode, context: BiomeRuleContext): void {
    this.checkNode(node, context);
  }

  /**
   * Visit HTML Element nodes
   */
  visitHTMLElement(node: BiomeNode, context: BiomeRuleContext): void {
    this.checkNode(node, context);
  }

  /**
   * Visit Vue Element nodes
   */
  visitVueElement(node: BiomeNode, context: BiomeRuleContext): void {
    this.checkNode(node, context);
  }

  /**
   * Main entry point for the rule
   */
  static create(context: BiomeRuleContext) {
    const rule = new UseSortedClassesRule();
    
    return {
      visitJSXElement: (node: BiomeNode) => rule.visitJSXElement(node, context),
      visitHTMLElement: (node: BiomeNode) => rule.visitHTMLElement(node, context),
      visitVueElement: (node: BiomeNode) => rule.visitVueElement(node, context),
    };
  }
}

/**
 * Export rule configuration for Biome
 */
export const useSortedClassesRule = {
  name: UseSortedClassesRule.ruleName,
  category: UseSortedClassesRule.category,
  description: UseSortedClassesRule.description,
  fixable: UseSortedClassesRule.fixable,
  create: UseSortedClassesRule.create,
};