use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use biome_tailwind_sorter::{TailwindFormatter, sort_tailwind_classes};
use std::time::Duration;

fn generate_test_content(num_components: usize, classes_per_component: usize) -> String {
    let class_pool = [
        "text-red-500", "p-4", "flex", "bg-blue-500", "m-2", "hover:bg-red-600",
        "rounded-lg", "font-semibold", "text-white", "inline-block", "border",
        "shadow-lg", "transition-colors", "duration-200", "ease-in-out",
        "transform", "hover:scale-105", "focus:outline-none", "focus:ring-2",
        "focus:ring-blue-500", "focus:ring-opacity-50", "absolute", "relative",
        "top-0", "left-0", "right-0", "bottom-0", "w-full", "h-full",
        "min-w-0", "max-w-none", "min-h-0", "max-h-none", "overflow-hidden",
        "overflow-auto", "text-center", "text-left", "text-right", "uppercase",
        "lowercase", "capitalize", "normal-case", "tracking-wide", "leading-tight",
        "space-x-2", "space-y-2", "divide-x", "divide-y", "border-solid",
        "border-dashed", "border-dotted", "border-double", "border-none",
    ];
    
    let mut content = String::new();
    for i in 0..num_components {
        content.push_str(&format!(r#"<div className=""#));
        
        // Add classes in reverse order to ensure they need sorting
        for j in (0..classes_per_component).rev() {
            let class_idx = (i * classes_per_component + j) % class_pool.len();
            content.push_str(class_pool[class_idx]);
            if j > 0 {
                content.push(' ');
            }
        }
        
        content.push_str(&format!(r#"">Component {}</div>"#, i));
        content.push('\n');
    }
    content
}

fn generate_large_file_content() -> String {
    // Simulate a large React component file
    format!(r#"
import React from 'react';

export const LargeComponent = () => {{
  return (
    <div className="container mx-auto px-4">
      {}
      <section className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 p-6">
        {}
      </section>
      <footer className="bg-gray-800 text-white p-8 mt-12">
        {}
      </footer>
    </div>
  );
}};
"#, 
    generate_test_content(50, 8),
    generate_test_content(100, 12),
    generate_test_content(20, 6)
    )
}

fn bench_sort_classes(c: &mut Criterion) {
    let small_classes = vec![
        "text-red-500".to_string(), "p-4".to_string(), "flex".to_string(),
        "bg-blue-500".to_string(), "m-2".to_string()
    ];
    
    let medium_classes = vec![
        "text-red-500", "p-4", "flex", "bg-blue-500", "m-2", "hover:bg-red-600",
        "rounded-lg", "font-semibold", "text-white", "inline-block", "border",
        "shadow-lg", "transition-colors", "duration-200", "ease-in-out",
        "transform", "hover:scale-105", "focus:outline-none", "focus:ring-2",
        "focus:ring-blue-500", "focus:ring-opacity-50"
    ].iter().map(|s| s.to_string()).collect();
    
    let large_classes = (0..100).map(|i| {
        let class_pool = [
            "text-red-500", "p-4", "flex", "bg-blue-500", "m-2", "hover:bg-red-600",
            "rounded-lg", "font-semibold", "text-white", "inline-block"
        ];
        class_pool[i % class_pool.len()].to_string()
    }).collect();
    
    let mut group = c.benchmark_group("sort_classes");
    
    group.bench_with_input(
        BenchmarkId::new("small", 5),
        &small_classes,
        |b, classes| b.iter(|| sort_tailwind_classes(black_box(classes)))
    );
    
    group.bench_with_input(
        BenchmarkId::new("medium", 20),
        &medium_classes,
        |b, classes| b.iter(|| sort_tailwind_classes(black_box(classes)))
    );
    
    group.bench_with_input(
        BenchmarkId::new("large", 100),
        &large_classes,
        |b, classes| b.iter(|| sort_tailwind_classes(black_box(classes)))
    );
    
    group.finish();
}

fn bench_format_document(c: &mut Criterion) {
    let formatter = TailwindFormatter::new(false);
    let formatter_with_cursor = TailwindFormatter::new(true);
    
    let small_content = generate_test_content(10, 5);
    let medium_content = generate_test_content(50, 8);
    let large_content = generate_large_file_content();
    
    let mut group = c.benchmark_group("format_document");
    group.measurement_time(Duration::from_secs(10));
    
    group.bench_with_input(
        BenchmarkId::new("small_no_cursor", "10_components"),
        &small_content,
        |b, content| b.iter(|| formatter.format_document(black_box(content), None))
    );
    
    group.bench_with_input(
        BenchmarkId::new("small_with_cursor", "10_components"),
        &small_content,
        |b, content| {
            let cursor_pos = Some(biome_tailwind_sorter::CursorPosition {
                line: 5,
                column: 10,
                offset: 100,
            });
            b.iter(|| formatter_with_cursor.format_document(black_box(content), cursor_pos.clone()))
        }
    );
    
    group.bench_with_input(
        BenchmarkId::new("medium_no_cursor", "50_components"),
        &medium_content,
        |b, content| b.iter(|| formatter.format_document(black_box(content), None))
    );
    
    group.bench_with_input(
        BenchmarkId::new("large_no_cursor", "large_file"),
        &large_content,
        |b, content| b.iter(|| formatter.format_document(black_box(content), None))
    );
    
    group.finish();
}

fn bench_regex_compilation(c: &mut Criterion) {
    use regex::Regex;
    use std::sync::LazyLock;
    
    static STATIC_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"(class(?:Name)?=")([^"]*?)""#).unwrap()
    });
    
    let test_content = generate_test_content(100, 10);
    
    let mut group = c.benchmark_group("regex_performance");
    
    group.bench_function("dynamic_compilation", |b| {
        b.iter(|| {
            let regex = Regex::new(r#"(class(?:Name)?=")([^"]*?)""#).unwrap();
            regex.find_iter(black_box(&test_content)).count()
        })
    });
    
    group.bench_function("static_compilation", |b| {
        b.iter(|| {
            STATIC_REGEX.find_iter(black_box(&test_content)).count()
        })
    });
    
    group.finish();
}

fn bench_file_processing_simulation(c: &mut Criterion) {
    let formatter = TailwindFormatter::new(false);
    
    // Simulate processing multiple files of different sizes
    let files = vec![
        ("small.tsx", generate_test_content(5, 4)),
        ("medium.jsx", generate_test_content(25, 6)),
        ("large.vue", generate_test_content(100, 8)),
        ("huge.html", generate_large_file_content()),
    ];
    
    let mut group = c.benchmark_group("file_processing");
    group.measurement_time(Duration::from_secs(15));
    
    for (filename, content) in &files {
        group.bench_with_input(
            BenchmarkId::new("single_file", *filename),
            content,
            |b, content| b.iter(|| formatter.format_document(black_box(content), None))
        );
    }
    
    // Benchmark processing all files together (simulates directory processing)
    group.bench_function("batch_processing", |b| {
        b.iter(|| {
            for (_, content) in black_box(&files) {
                formatter.format_document(content, None);
            }
        })
    });
    
    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let formatter = TailwindFormatter::new(false);
    
    // Test with increasingly large inputs to see memory scaling
    let sizes = [100, 500, 1000, 5000];
    
    let mut group = c.benchmark_group("memory_scaling");
    group.measurement_time(Duration::from_secs(10));
    
    for &size in &sizes {
        let content = generate_test_content(size, 10);
        group.bench_with_input(
            BenchmarkId::new("components", size),
            &content,
            |b, content| b.iter(|| formatter.format_document(black_box(content), None))
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_sort_classes,
    bench_format_document,
    bench_regex_compilation,
    bench_file_processing_simulation,
    bench_memory_usage
);
criterion_main!(benches);